# swf-monitor

Monitor and control
[`slurmworkflow`](https://github.com/EpiModel/slurmworkflow)s on a SLURM
equiped HPC

## Next step

- add a second tab to inspect a specific workflows
    - select them with summary in tab1
    - in tab2, can go through the steps
- first add the `tab` widget in the header
- then implement the second tab
- look at gitui for insipration?

## To Do

- [x] dummy layout
- [ ] allow the selection of one of the wfs
- list workflows
    - `find ./ -name "start_workflow.sh" -type f`
    - manage file paths with: https://doc.rust-lang.org/std/path/struct.Path.html
    - search for the "SWF/controller.sh" to ensure it's a wf
- get wf infos
- list wf logs
- show logs by step easily get last?
- read logs
- start workflow
- stop / cancel workflow
- show wf status (running, etc)

- find a way to use it locally
    - send cmd through ssh
    - handle ssh connection
- port to a web app?
    - leptos, yew, dioxus
    - wasm
