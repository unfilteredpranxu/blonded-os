#!/usr/bin/env python3
"""Bounded real-client test; only launches/stops its own nested processes."""
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import tempfile
import time


def wait_for(process, predicate, timeout):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return
        if process.poll() is not None:
            raise RuntimeError(f"compositor exited early: {process.returncode}")
        time.sleep(0.1)
    raise TimeoutError("nested smoke condition timed out")


def main():
    root = Path(__file__).resolve().parent.parent
    binaries = root / "crates/target/debug"
    evidence = Path(tempfile.mkdtemp(prefix="blonded-smoke-", dir="/tmp"))
    server_log = evidence / "compositor.log"
    client_log = evidence / "lifecycle.log"
    capture = evidence / "framebuffer.ppm"
    print(f"Evidence: {evidence}", flush=True)
    server = None
    client = None
    try:
        with server_log.open("w") as log:
            server = subprocess.Popen(
                [str(binaries / "blonded-compositor"), "--nested", "--smoke-test",
                 "--exit-after", "40", "--capture", str(capture)],
                stdout=log, stderr=subprocess.STDOUT,
            )
        read_log = lambda: server_log.read_text()
        # Software Mesa initialization on the target HDD can be slow. This
        # timeout includes initialization; the server's 40s timer starts later.
        wait_for(server, lambda: "RENDER submitted_with_client=true" in read_log()
                 and "SMOKE frame_callback" in read_log(), 90)
        socket = Path(re.search(r"READY socket=(\S+)", read_log()).group(1))
        directory_mode = socket.parent.stat().st_mode & 0o777
        assert directory_mode == 0o700, f"unsafe socket directory: {oct(directory_mode)}"
        assert len(re.findall(r"^MAP id=", read_log(), re.M)) >= 2
        assert capture.exists() and capture.stat().st_size > 100
        assert "SMOKE frame_callback" in read_log(), "no client frame callbacks"
        environment = dict(os.environ, WAYLAND_DISPLAY=str(socket))
        environment.pop("WAYLAND_SOCKET", None)
        with client_log.open("w") as log:
            client = subprocess.Popen(
                [str(binaries / "blonded-smoke-client"), "lifecycle", "--lifecycle"],
                env=environment, stdout=log, stderr=subprocess.STDOUT,
            )
        assert client.wait(timeout=8) == 0, "lifecycle client failed"
        wait_for(server, lambda: "UNMAP id=4 remaining=2" in read_log(), 3)
        assert "MAP id=3" in read_log() and "MAP id=4" in read_log()
        assert "SMOKE unmap" in client_log.read_text()
        assert "SMOKE remap_requested" in client_log.read_text()
        assert "SMOKE keyboard_enter" in client_log.read_text()
        assert "CLIENT disconnected" in read_log()

        # A short process-only sample; neither whole-system idle nor a benchmark.
        def ticks():
            fields = Path(f"/proc/{server.pid}/stat").read_text().split(") ", 1)[1].split()
            return int(fields[11]) + int(fields[12])

        start = time.monotonic()
        initial_ticks = ticks()
        time.sleep(2)
        elapsed = time.monotonic() - start
        cpu_ticks = ticks() - initial_ticks
        status = Path(f"/proc/{server.pid}/status").read_text()
        rss_kib = int(re.search(r"VmRSS:\s+(\d+) kB", status).group(1))
        server.send_signal(signal.SIGINT)
        assert server.wait(timeout=8) == 0, "SIGINT shutdown failed"
        assert "STOP windows=0 invariants=Ok(())" in read_log()
        assert not socket.exists() and not socket.parent.exists(), "socket leaked"
        renderer = re.search(r'GL Renderer: "([^"]+)"', read_log())
        summary = {
            "result": "PASS", "smithay": "0.7.0", "socket_directory_mode": oct(directory_mode),
            "real_clients": 3, "lifecycle_map_ids": [3, 4], "shutdown": "SIGINT exit 0",
            "framebuffer_readback": True, "physical_presentation": "NOT VERIFIED",
            "physical_pointer_keyboard": "NOT VERIFIED",
            "renderer": renderer.group(1) if renderer else "UNKNOWN",
            "process_rss_kib": rss_kib, "sample_seconds": round(elapsed, 3),
            "process_cpu_ticks": cpu_ticks, "clock_ticks_per_second": os.sysconf("SC_CLK_TCK"),
        }
        (evidence / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
        print(json.dumps(summary, indent=2), flush=True)
    finally:
        for process in [client, server]:
            if process is not None and process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=8)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


if __name__ == "__main__":
    main()
