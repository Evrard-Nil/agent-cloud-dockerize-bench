# Auto-Dockerizer Benchmark

This tool benchmarks an auto-dockerizer's ability to successfully containerize various AI agent repositories.

## Installation

To install the benchmark tool, use `cargo`:

```bash
cargo install auto-dockerizer-benchmark
```

## Usage

The benchmark tool takes a single command as input, which should be the command to run your auto-dockerizer on a given repository. The benchmark will iterate through a predefined list of AI agent repositories, execute your provided command for each, and then prompt you to judge the success or failure of the dockerization.

```bash
auto-dockerizer-benchmark <YOUR_AUTO_DOCKERIZER_COMMAND_HERE>
```

**Example:**

```bash
auto-dockerizer-benchmark "my-dockerizer build"
```

## The benchmark

The list of repositories include the following tools and frameworks:

 - [X] Python no frameworks
 - [ ] Langchain (Python script)
 - [ ] Langchain (Notebook)
 - [ ] LlamaIndex (Python script)
 - [ ] LlamaIndex (Notebook)
 - [ ] AutoGen (Python script)


## Report sample

```
--- Auto-Dockerizer Benchmark Report ---

Overall Performance:
  Total Repositories Benchmarked: 3
  Successful Dockerizations: 2
  Failed Dockerizations: 1
  Overall Success Rate: 66.67%

Breakdown by Tag:
  Python:
    Successful: 2
    Failed: 0
    Success Rate: 100.00%
  Langchain:
    Successful: 1
    Failed: 0
    Success Rate: 100.00%
  Notebook:
    Successful: 0
    Failed: 1
    Success Rate: 0.00%
  LlamaIndex:
    Successful: 0
    Failed: 1
    Success Rate: 0.00%

--- End Report ---
```



## Contributing a Repository

Contributions of new AI agent repositories to expand the benchmark's coverage are welcome. If you have a repository that you'd like to add, please consider opening a Pull Request.

To contribute:

1.  Fork this repository.
2.  Add your repository's URL to the `repositories.json` file with tags.
3.  Ensure your repository is publicly accessible and contains a typical AI agent setup (e.g., Python script, Jupyter notebook, dependency files).
4.  Submit a Pull Request.