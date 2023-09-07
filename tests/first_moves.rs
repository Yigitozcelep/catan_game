use catan_game::game::game_controller::GameController;

#[test]
fn check_first_moves() {
    for _ in 0..1000 {
        let data = GameController::random_map();
        println!("{:?}", data.get_all_housable_point().len());
        for el in data.get_all_housable_point() {
            if !data.state.map[el.0][el.1].is_housable() {panic!()}
        }
    }
}
