#!/usr/bin/env raku
sub dec_table(@list --> Str:D) {
    my @a is default(0xFFFF);
    my @pairs = @list.map(*.NFC[0]).pairs.map(*.antipair);
    for @pairs { @a[.key] = .value };
    "[" ~ @a.map({"0x" ~ .base(16)}).join(",\n") ~ "]"
}

sub enc_table(@list --> Str:D) {
    "[" ~ @list.map({ “'$_'” }).join(",\n") ~ "]"
}

sub MAIN {
    my @list = open("base2048.txt").slurp.subst("\n","", :g).comb();
    open("./src/enc_table.src", :w).spurt(enc_table(@list));
    open("./src/dec_table.src", :w).spurt(dec_table(@list));
}
