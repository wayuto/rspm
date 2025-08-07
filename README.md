# ***Rspm***
*rspm is a command-line tool for managing processes, allowing users to view, pause, resume, and terminate processes.*

## ***Building***
```bash
git clone https://github.com/wayuto/rspm.git ~/rspm --depth 1
cd ~/rspm
cargo build
```

## ***Usage***
```bash
Usage: rspm <COMMAND>

Commands:
  proc  Show all the processes of current oprating system
  kbp   Kill a process by its PID
  kbn   Kill a process by its name
  pbp   Pause a process by its PID
  pbn   Pause a process by its name
  rbp   Resume a process by its PID
  rbn   Resume a process by its name
  top   Show Processes like top
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## ***Contributing***
*Contributions are welcome! Please fork this repository and submit a pull request.*

## ***License***
*This project is licensed under the GPLv3 License.*
