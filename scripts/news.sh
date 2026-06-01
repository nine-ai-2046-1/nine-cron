NEWS=$(nine-poe --model "RealTimeNewsHK" --prompt "summrise in 2 paragraph, use double \n for \n") && opencb send "$NEWS"
