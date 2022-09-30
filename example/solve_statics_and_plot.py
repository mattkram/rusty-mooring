from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits import mplot3d  # noqa

from rusty_mooring import MooringSystem
from rusty_mooring import Node


def plot_results(results: dict[str, list[Node]]) -> None:
    plt.figure()
    ax = plt.axes(projection="3d")
    for line_name, nodes in results.items():
        coords = np.array([[node.coords.x, node.coords.y, node.coords.z] for node in nodes])
        ax.plot3D(coords[:, 0], coords[:, 1], coords[:, 2])
    ax.set_xlabel("x [m]")
    ax.set_ylabel("y [m]")
    ax.set_zlabel("z [m]")
    ax.set_xlim([-1500.0, 1500.0])
    ax.set_ylim([-1500.0, 1500.0])
    plt.show()


def run() -> None:
    system = MooringSystem.from_file((Path(__file__).parent / "system.toml").as_posix())
    results = system.solve_static()
    plot_results(results)


if __name__ == "__main__":
    run()
