use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    START
}

#[derive(Debug, Clone)]
struct Node {
    node_index: usize,
    prev_node_index: usize,

    length: usize,

    blank_index: u8,
    board_state: Vec<u8>,
    direction: Direction
}

impl Node {
    fn can_move_right(&self) -> bool {
        self.blank_index % 3 != 2
    }
    fn can_move_left(&self) -> bool {
        self.blank_index % 3 != 0
    }
    fn can_move_up(&self) -> bool {
        self.blank_index > 2
    }
    fn can_move_down(&self) -> bool {
        self.blank_index < 6
    }

    fn move_right(&self, all_states: &mut Vec<Box<Node>>) -> usize {
        let mut new_board_state = self.board_state.clone();
        new_board_state.swap(
            self.blank_index as usize,
            (self.blank_index as usize) + 1
        );

        let node_index = all_states.len();
        all_states.push(Box::new(Node {
            node_index,
            prev_node_index: self.node_index,
            length: self.length + 1,
            blank_index: self.blank_index + 1,
            board_state: new_board_state,
            direction: Direction::RIGHT
        }));

        return node_index;
    }

    fn move_left(&self, all_states: &mut Vec<Box<Node>>) -> usize {
        let mut new_board_state = self.board_state.clone();
        new_board_state.swap(
            self.blank_index as usize,
            (self.blank_index as usize) - 1
        );

        let node_index = all_states.len();
        all_states.push(Box::new(Node {
            node_index,
            prev_node_index: self.node_index,
            length: self.length + 1,
            blank_index: self.blank_index - 1,
            board_state: new_board_state,
            direction: Direction::LEFT
        }));

        return node_index;
    }

    fn move_up(&self, all_states: &mut Vec<Box<Node>>) -> usize {
        let mut new_board_state = self.board_state.clone();
        new_board_state.swap(
            self.blank_index as usize,
            (self.blank_index as usize) - 3
        );

        let node_index = all_states.len();
        all_states.push(Box::new(Node {
            node_index,
            prev_node_index: self.node_index,
            length: self.length + 1,
            blank_index: self.blank_index - 3,
            board_state: new_board_state,
            direction: Direction::UP
        }));

        return node_index;
    }

    fn move_down(&self, all_states: &mut Vec<Box<Node>>) -> usize {
        let mut new_board_state = self.board_state.clone();
        new_board_state.swap(
            self.blank_index as usize,
            (self.blank_index as usize) + 3
        );

        let node_index = all_states.len();
        all_states.push(Box::new(Node {
            node_index,
            prev_node_index: self.node_index,
            length: self.length + 1,
            blank_index: self.blank_index + 3,
            board_state: new_board_state,
            direction: Direction::DOWN
        }));

        return node_index;
    }

    fn print_board_state(&self) {
        println!(
            "{}|{}|{}\n{}|{}|{}\n{}|{}|{}",
            self.board_state[0],
            self.board_state[1],
            self.board_state[2],
            self.board_state[3],
            self.board_state[4],
            self.board_state[5],
            self.board_state[6],
            self.board_state[7],
            self.board_state[8],
        )
    }
}

fn main() {
    let goal_board_state = vec!(            
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        0
    );
    let initial_state = Box::new(Node {
        prev_node_index: 0,
        node_index: 0,
        length: 0,
        blank_index: 8,
        board_state: vec!(1,4,5,8,2,7,3,6,0),
        direction: Direction::START
    });

    let mut all_states: Vec<Box<Node>> = Vec::new();
    let mut open_states: VecDeque<usize> = VecDeque::new();
    let mut closed_states: HashSet<Vec<u8>> = HashSet::new();
    let mut winning_state: usize = 0;

    all_states.push(initial_state);
    open_states.push_back(0);

    while !open_states.is_empty(){
        let current_state = open_states.pop_front().unwrap();
        let current_node = all_states[current_state].clone(); 

        if closed_states.contains(&current_node.board_state) {
            continue;
        }

        if goal_board_state == current_node.board_state {
            winning_state = current_state;
            break;
        }

        // Enumerate the possible next states
        if current_node.can_move_right() {
            open_states.push_back(current_node.move_right(&mut all_states));
        }

        if current_node.can_move_left() {
            open_states.push_back(current_node.move_left(&mut all_states));
        }

        if current_node.can_move_up() {
            open_states.push_back(current_node.move_up(&mut all_states));
        }

        if current_node.can_move_down() {
            open_states.push_back(current_node.move_down(&mut all_states));
        }


        closed_states.insert(current_node.board_state);
    }

    let winning_node = all_states[winning_state].clone();
    let mut current_node = all_states[winning_state].clone();

    while current_node.prev_node_index != 0 {
        current_node.print_board_state();
        println!("Direction: {:?}", current_node.direction);
        current_node = all_states[current_node.prev_node_index].clone();
    }
    all_states[0].print_board_state();

    println!("Found solution using {} moves for a pathlength of {}", winning_state, winning_node.length);
    //println!("{:#?}", all_states);

}
