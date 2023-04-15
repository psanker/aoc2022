let read_lines file_name : string list =
    let ic = open_in file_name in
    let try_read () =
        try 
            Some (input_line ic) 
        with End_of_file -> 
            None 
    in let rec loop acc = 
        match try_read () with
        | Some line -> loop (line :: acc)
        | None -> close_in ic; List.rev acc in
    loop []


