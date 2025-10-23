fn t() {
    match get!(0) {
        'a' => check!(1, "", A),
        'b' => match get!(1, B) {
            'g' => check!(2, "", X),
            'l' => match get!(2) {
                'a' => check!(3, "ck", BLACK),
                'i' => check!(3, "nk", K),
                'o' => check!(3, "ck", BLOCK),
                'u' => check!(3, "e", BLUE),
                _ => {}
            },
            'r' => check!(2, "", BR),
            _ => {}
        },
        'c' => match get!(1, C) {
            'l' => check!(2, "ass", CLASS),
            'u' => check!(2, "rly", CURLY),
            'y' => check!(2, "an", CYAN),
            _ => {}
        },
        'd' => match get!(1, D) {
            'a' => check!(2, "shed", CYAN),
            'i' => match get!(2) {
                'm' => check!(3, "", D),
                'v' => check!(3, "", DIV),
                _ => {}
            },
            'o' => match get!(2) {
                't' => check!(3, "ted", DOTTED),
                'u' => match get!(3) {
                    'b' => match get!(4) {
                        'l' => match get!(5) {
                            'e' => match get!(6, DOUBLE) {
                                '-' => match get!(7) {
                                    'u' => match get!(8) {
                                        'n' => match get!(9) {
                                            'd' => match get!(10) {
                                                'e' => match get!(11) {
                                                    'r' => match get!(12, U) {
                                                        'l' => check!(3, "ine", UU),
                                                        _ => {}
                                                    },
                                                    _ => {}
                                                },
                                                _ => {}
                                            },
                                            _ => {}
                                        },
                                        _ => {}
                                    },
                                    _ => {}
                                },
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        'e' => check!(1, "m", I),
        'f' => match get!(1) {
            'i' => check!(2, "xed", FIXED),
            'g' => check!(2, "", C),
            _ => {}
        },
        'g' => check!(1, "reen", GREEN),
        'h' => match get!(1, H) {
            'i' => match get!(2) {
                'd' => match get!(3) {
                    'd' => check!(4, "en", H),
                    'e' => check!(4, "", H),
                    _ => {}
                },
                _ => {}
            },
            'r' => check!(2, "ef", HREF),
            _ => {}
        },
        'i' => match get!(1, I) {
            'd' => check!(2, "", ID),
            'n' => match get!(2) {
                'd' => check!(3, "ent", INDENT),
                's' => check!(3, "", U),
                'v' => match get!(3) {
                    'e' => check!(4, "rt", R),
                    'i' => check!(4, "sible", H),
                    _ => {}
                },
                _ => {}
            },
            't' => check!(2, "alics", I),
            _ => {}
        },
        'k' => check!(1, "", K),
        'l' => check!(1, "et", LET),
        'm' => check!(1, "agenta", MAGENTA),
        'n' => match get!(1, N) {
            'e' => check!(2, "gative", R),
            'o' => check!(2, "ne", NONE),
            _ => {}
        },
        'p' => match get!(1, P) {
            'r' => check!(2, "e", PRE),
            _ => {}
        },
        'r' => match get!(1, R) {
            'e' => match get!(2) {
                'd' => check!(3, "", RED),
                'v' => check!(3, "erse", R),
                _ => {}
            },
            'g' => check!(2, "b", RGB),
            _ => {}
        },
        's' => match get!(1, S) {
            'i' => check!(2, "ngle", SINGLE),
            'p' => check!(2, "an", SPAN),
            't' => match get!(2) {
                'r' => match get!(3) {
                    'i' => match get!(4) {
                        'k' => match get!(5) {
                            'e' => match get!(6, S) {
                                '-' => check!(7, "through", S),
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        },
        'u' => match get!(1, U) {
            'n' => match get!(2) {
                'd' => match get!(3) {
                    'e' => match get!(4) {
                        'r' => match get!(5, U) {
                            'l' => check!(6, "ine", U),
                            _ => {}
                        },
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
            'u' => check!(2, "", UU),
            _ => {}
        },
        'w' => check!(1, "hite", WHITE),
        'x' => check!(1, "", X),
        'y' => check!(1, "ellow", YELLOW),
        'z' => check!(1, "iyy", ZIYY),
        _ => {}
    }
}
