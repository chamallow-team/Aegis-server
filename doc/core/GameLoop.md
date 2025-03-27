**Game loop**

<!-- TOC -->

* [Usage](#usage)
* [Flow](#flow)

<!-- TOC -->

## Usage

The game loop is the main part of the server. It'll control how each task will be run and in which order.

## Flow

![flow](../assets/game_loop.drawio.svg)

## Parallel processing organisation

In this flow, we'll need a lot of parallel tasks to run, such as the API, the core, the backup system, and so on.

Each main part will each have his own thread. And in this thread, it'll use green-threads and technics such as a
worker-pool (or thread-pool).

## Communication

