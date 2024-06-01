use crate::game::Game;

#[test]
fn ivanchuk_vs_yusupov() {
    let mut game = Game::new();

    game.validate_and_make_move("c2", "c4", ' ').expect("c4");
    assert_eq!(game.previous_move, "c4");

    game.validate_and_make_move("e7", "e5", ' ').expect("e5");
    assert_eq!(game.previous_move, "e5");

    game.validate_and_make_move("g2", "g3", ' ').expect("g3");
    assert_eq!(game.previous_move, "g3");

    game.validate_and_make_move("d7", "d6", ' ').expect("d6");
    assert_eq!(game.previous_move, "d6");

    game.validate_and_make_move("f1", "g2", ' ').expect("Bg2");
    assert_eq!(game.previous_move, "Bg2");

    game.validate_and_make_move("g7", "g6", ' ').expect("g6");
    assert_eq!(game.previous_move, "g6");

    game.validate_and_make_move("d2", "d4", ' ').expect("d4");
    assert_eq!(game.previous_move, "d4");

    game.validate_and_make_move("b8", "d7", ' ').expect("Nd7");
    assert_eq!(game.previous_move, "Nd7");

    game.validate_and_make_move("b1", "c3", ' ').expect("Nc3");
    assert_eq!(game.previous_move, "Nc3");

    game.validate_and_make_move("f8", "g7", ' ').expect("Bg7");
    assert_eq!(game.previous_move, "Bg7");

    game.validate_and_make_move("g1", "f3", ' ').expect("Nf3");
    assert_eq!(game.previous_move, "Nf3");

    game.validate_and_make_move("g8", "f6", ' ').expect("Nf6");
    assert_eq!(game.previous_move, "Nf6");

    game.validate_and_make_move("e1", "g1", ' ').expect("0-0");
    assert_eq!(game.previous_move, "0-0");

    game.validate_and_make_move("e8", "g8", ' ').expect("0-0");
    assert_eq!(game.previous_move, "0-0");

    game.validate_and_make_move("d1", "c2", ' ').expect("Qc2");
    assert_eq!(game.previous_move, "Qc2");

    game.validate_and_make_move("f8", "e8", ' ').expect("Re8");
    assert_eq!(game.previous_move, "Re8");

    game.validate_and_make_move("f1", "d1", ' ').expect("Rd1");
    assert_eq!(game.previous_move, "Rd1");

    game.validate_and_make_move("c7", "c6", ' ').expect("c6");
    assert_eq!(game.previous_move, "c6");

    game.validate_and_make_move("b2", "b3", ' ').expect("b3");
    assert_eq!(game.previous_move, "b3");

    game.validate_and_make_move("d8", "e7", ' ').expect("Qe7");
    assert_eq!(game.previous_move, "Qe7");

    game.validate_and_make_move("c1", "a3", ' ').expect("Ba3");
    assert_eq!(game.previous_move, "Ba3");

    game.validate_and_make_move("e5", "e4", ' ').expect("e4");
    assert_eq!(game.previous_move, "e4");

    game.validate_and_make_move("f3", "g5", ' ').expect("Ng5");
    assert_eq!(game.previous_move, "Ng5");

    game.validate_and_make_move("e4", "e3", ' ').expect("e3");
    assert_eq!(game.previous_move, "e3");

    game.validate_and_make_move("f2", "f4", ' ').expect("f4");
    assert_eq!(game.previous_move, "f4");

    game.validate_and_make_move("d7", "f8", ' ').expect("Nf8");
    assert_eq!(game.previous_move, "Nf8");

    game.validate_and_make_move("b3", "b4", ' ').expect("b4");
    assert_eq!(game.previous_move, "b4");

    game.validate_and_make_move("c8", "f5", ' ').expect("Bf5");
    assert_eq!(game.previous_move, "Bf5");

    game.validate_and_make_move("c2", "b3", ' ').expect("Qb3");
    assert_eq!(game.previous_move, "Qb3");

    game.validate_and_make_move("h7", "h6", ' ').expect("h6");
    assert_eq!(game.previous_move, "h6");

    game.validate_and_make_move("g5", "f3", ' ').expect("Nf3");
    assert_eq!(game.previous_move, "Nf3");

    game.validate_and_make_move("f6", "g4", ' ').expect("Ng4");
    assert_eq!(game.previous_move, "Ng4");

    game.validate_and_make_move("b4", "b5", ' ').expect("b5");
    assert_eq!(game.previous_move, "b5");

    game.validate_and_make_move("g6", "g5", ' ').expect("g5");
    assert_eq!(game.previous_move, "g5");

    game.validate_and_make_move("b5", "c6", ' ').expect("xc6");
    assert_eq!(game.previous_move, "xc6");

    game.validate_and_make_move("b7", "c6", ' ').expect("xc6");
    assert_eq!(game.previous_move, "xc6");

    game.validate_and_make_move("f3", "e5", ' ').expect("Ne5");
    assert_eq!(game.previous_move, "Ne5");

    game.validate_and_make_move("g5", "f4", ' ').expect("xf4");
    assert_eq!(game.previous_move, "xf4");

    game.validate_and_make_move("e5", "c6", ' ').expect("Nxc6");
    assert_eq!(game.previous_move, "Nxc6");

    game.validate_and_make_move("e7", "g5", ' ').expect("Qg5");
    assert_eq!(game.previous_move, "Qg5");

    game.validate_and_make_move("a3", "d6", ' ').expect("Bxd6");
    assert_eq!(game.previous_move, "Bxd6");

    game.validate_and_make_move("f8", "g6", ' ').expect("Ng6");
    assert_eq!(game.previous_move, "Ng6");

    game.validate_and_make_move("c3", "d5", ' ').expect("Nd5");
    assert_eq!(game.previous_move, "Nd5");

    game.validate_and_make_move("g5", "h5", ' ').expect("Qh5");
    assert_eq!(game.previous_move, "Qh5");

    game.validate_and_make_move("h2", "h4", ' ').expect("h4");
    assert_eq!(game.previous_move, "h4");

    game.validate_and_make_move("g6", "h4", ' ').expect("Nxh4");
    assert_eq!(game.previous_move, "Nxh4");

    game.validate_and_make_move("g3", "h4", ' ').expect("xh4");
    assert_eq!(game.previous_move, "xh4");

    game.validate_and_make_move("h5", "h4", ' ').expect("Qxh4");
    assert_eq!(game.previous_move, "Qxh4");

    game.validate_and_make_move("d5", "e7", ' ').expect("Ne7+");
    assert_eq!(game.previous_move, "Ne7+");

    game.validate_and_make_move("g8", "h8", ' ').expect("Kh8");
    assert_eq!(game.previous_move, "Kh8");

    game.validate_and_make_move("e7", "f5", ' ').expect("Nxf5");
    assert_eq!(game.previous_move, "Nxf5");

    game.validate_and_make_move("h4", "h2", ' ').expect("Qh2+");
    assert_eq!(game.previous_move, "Qh2+");

    game.validate_and_make_move("g1", "f1", ' ').expect("Kf1");
    assert_eq!(game.previous_move, "Kf1");

    game.validate_and_make_move("e8", "e6", ' ').expect("Re6");
    assert_eq!(game.previous_move, "Re6");

    game.validate_and_make_move("b3", "b7", ' ').expect("Qb7");
    assert_eq!(game.previous_move, "Qb7");

    game.validate_and_make_move("e6", "g6", ' ').expect("Rg6");
    assert_eq!(game.previous_move, "Rg6");

    game.validate_and_make_move("b7", "a8", ' ').expect("Qxa8+");
    assert_eq!(game.previous_move, "Qxa8+");

    game.validate_and_make_move("h8", "h7", ' ').expect("Kh7");
    assert_eq!(game.previous_move, "Kh7");

    game.validate_and_make_move("a8", "g8", ' ').expect("Qg8+");
    assert_eq!(game.previous_move, "Qg8+");

    game.validate_and_make_move("h7", "g8", ' ').expect("Kxg8");
    assert_eq!(game.previous_move, "Kxg8");

    game.validate_and_make_move("c6", "e7", ' ').expect("Ne7+");
    assert_eq!(game.previous_move, "Ne7+");

    game.validate_and_make_move("g8", "h7", ' ').expect("Kh7");
    assert_eq!(game.previous_move, "Kh7");

    game.validate_and_make_move("e7", "g6", ' ').expect("Nxg6");
    assert_eq!(game.previous_move, "Nxg6");

    game.validate_and_make_move("f7", "g6", ' ').expect("xg6");
    assert_eq!(game.previous_move, "xg6");

    game.validate_and_make_move("f5", "g7", ' ').expect("Nxg7");
    assert_eq!(game.previous_move, "Nxg7");

    game.validate_and_make_move("g4", "f2", ' ').expect("Nf2");
    assert_eq!(game.previous_move, "Nf2");

    game.validate_and_make_move("d6", "f4", ' ').expect("Bxf4");
    assert_eq!(game.previous_move, "Bxf4");

    game.validate_and_make_move("h2", "f4", ' ').expect("Qxf4");
    assert_eq!(game.previous_move, "Qxf4");

    game.validate_and_make_move("g7", "e6", ' ').expect("Ne6");
    assert_eq!(game.previous_move, "Ne6");

    game.validate_and_make_move("f4", "h2", ' ').expect("Qh2");
    assert_eq!(game.previous_move, "Qh2");

    game.validate_and_make_move("d1", "b1", ' ').expect("Rb1");
    assert_eq!(game.previous_move, "Rb1");

    game.validate_and_make_move("f2", "h3", ' ').expect("Nh3");
    assert_eq!(game.previous_move, "Nh3");

    game.validate_and_make_move("b1", "b7", ' ').expect("Rb7+");
    assert_eq!(game.previous_move, "Rb7+");

    game.validate_and_make_move("h7", "h8", ' ').expect("Kh8");
    assert_eq!(game.previous_move, "Kh8");

    game.validate_and_make_move("b7", "b8", ' ').expect("Rb8+");
    assert_eq!(game.previous_move, "Rb8+");

    game.validate_and_make_move("h2", "b8", ' ').expect("Qxb8");
    assert_eq!(game.previous_move, "Qxb8");

    game.validate_and_make_move("g2", "h3", ' ').expect("Bxh3");
    assert_eq!(game.previous_move, "Bxh3");

    game.validate_and_make_move("b8", "g3", ' ').expect("Qg3");
    assert_eq!(game.previous_move, "Qg3");
}
