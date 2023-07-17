open Core

let read_lines file_name =
    let file = In_channel.create file_name in
    let lines = In_channel.input_lines file in
    let () = In_channel.close file in
    lines
