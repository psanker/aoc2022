let max_cals input =
    let rec max_cals' input acc top = 
        match input with
        | [] -> top
        | "" :: rest -> 
            let new_top = if acc > top then acc else top
            in max_cals' rest 0 new_top
        | x :: rest -> max_cals' rest (acc + int_of_string x) top
    in max_cals' input 0 0

let top_three_cals input =
    let rec top_three_cals' input acc top_three =
        match input, top_three with
        | [], _ -> top_three
        | "" :: rest, m1 :: m2 :: m3 :: [] ->
            let new_top_three = 
                if acc > m1 then
                    acc :: m1 :: m2 :: []
                else if acc > m2 then
                    m1 :: acc :: m2 :: []
                else if acc > m3 then
                    m1 :: m2 :: acc :: []
                else
                    m1 :: m2 :: m3 :: []
            in top_three_cals' rest 0 new_top_three
        | x :: rest, y -> top_three_cals' rest (acc + int_of_string x) y
    in top_three_cals' input 0 [0; 0; 0;]

let run () =
    let lines = Advent.read_lines "inputs/day1.txt" in
    Printf.printf "%d\n" (max_cals lines);
    Printf.printf "%d\n" (List.fold_left (+) 0 @@ top_three_cals lines)
