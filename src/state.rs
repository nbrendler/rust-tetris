use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::time::{Duration, Instant};

use ggez::audio::SoundSource;
use ggez::event::{Axis, Button, EventHandler, GamepadId, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::constants::{
    ASPECT_RATIO, BASE_SPEED, COLUMNS, INPUT_DELAY, MOVEMENT_DELAY, ROWS, SPEED_PER_LEVEL,
};
use crate::piece::{create_random_piece, Block, Piece};
use crate::position::Position;
use crate::types::{Point2, Vec2};

struct GridState([[Option<Block>; COLUMNS]; ROWS]);

impl fmt::Display for GridState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                match self.0[i][j] {
                    Some(_) => write!(f, "x")?,
                    None => write!(f, "o")?,
                };
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct InputState {
    left: bool,
    right: bool,
    down: bool,
    up: bool,
    rotate_right: bool,
    rotate_left: bool,
    hold: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            left: false,
            right: false,
            down: false,
            up: false,
            rotate_right: false,
            rotate_left: false,
            hold: false,
        }
    }
}

impl InputState {
    fn moved(&self) -> bool {
        if vec![self.left, self.right, self.down].iter().any(|&x| x) {
            return true;
        }
        false
    }
    fn acted(&self) -> bool {
        if vec![self.up, self.rotate_left, self.rotate_right]
            .iter()
            .any(|&x| x)
        {
            return true;
        }
        false
    }
}

gfx_defines! {
    constant Opacity {
        pct: f32 = "u_Pct",

    }
}

type SharedState = Rc<RefCell<ContextBoundState>>;

trait Scene: EventHandler {
    fn shared_state(&self) -> SharedState;
    fn get_transition(&mut self) -> Option<Transition>;
}

enum TransitionType {
    Swap,
    Push,
    Pop,
    Reset,
}

struct Transition {
    scene: Option<Box<dyn Scene + 'static>>,
    transition_type: TransitionType,
}

struct GameOverScene {
    state: SharedState,
    restart: bool,
}

impl GameOverScene {
    fn new(state: &SharedState) -> Self {
        Self {
            state: state.clone(),
            restart: false,
        }
    }
}

impl EventHandler for GameOverScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let state = self.state.borrow();
        let (screen_w, screen_h) = graphics::size(ctx);

        let overlay = graphics::Rect::new(0.0, 0.0, screen_w, screen_h);
        let overlay_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            overlay,
            graphics::Color::new(0.0, 0.0, 0.0, 0.8),
        )?;
        graphics::draw(ctx, &overlay_rect, graphics::DrawParam::default())?;

        let title_str = "GAME OVER";
        let title_display = graphics::Text::new((title_str, state.assets.font, 24.0));
        let title_dest = Point2::new(
            screen_w / 2.0 - title_display.width(ctx) as f32 / 2.0,
            screen_h / 2.0,
        );
        let title_params = graphics::DrawParam::default()
            .dest(title_dest)
            .offset(Point2::new(0.5, 0.5));
        graphics::draw(ctx, &title_display, title_params)?;

        Ok(())
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        match btn {
            _ => self.restart = true,
        }
    }
}

impl Scene for GameOverScene {
    fn shared_state(&self) -> SharedState {
        self.state.clone()
    }

    fn get_transition(&mut self) -> Option<Transition> {
        if self.restart {
            return Some(Transition {
                transition_type: TransitionType::Reset,
                scene: Some(Box::new(GameScene::new(&self.shared_state()))),
            });
        }
        None
    }
}

struct PauseScene {
    state: SharedState,
    resume: bool,
}

impl PauseScene {
    fn new(state: &SharedState) -> Self {
        Self {
            state: state.clone(),
            resume: false,
        }
    }
}

impl EventHandler for PauseScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let state = self.state.borrow();
        let (screen_w, screen_h) = graphics::size(ctx);

        let overlay = graphics::Rect::new(0.0, 0.0, screen_w, screen_h);
        let overlay_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            overlay,
            graphics::Color::new(0.0, 0.0, 0.0, 0.8),
        )?;
        graphics::draw(ctx, &overlay_rect, graphics::DrawParam::default())?;

        let title_str = "PAUSED";
        let title_display = graphics::Text::new((title_str, state.assets.font, 24.0));
        let title_dest = Point2::new(
            screen_w / 2.0 - title_display.width(ctx) as f32 / 2.0,
            screen_h / 2.0,
        );
        let title_params = graphics::DrawParam::default()
            .dest(title_dest)
            .offset(Point2::new(0.5, 0.5));
        graphics::draw(ctx, &title_display, title_params)?;

        let instructions = "Press start to resume";
        let inst_display = graphics::Text::new((instructions, state.assets.font, 18.0));
        let inst_dest = Point2::new(
            screen_w / 2.0 - inst_display.width(ctx) as f32 / 2.0,
            screen_h / 2.0 + 50.0,
        );
        let inst_params = graphics::DrawParam::default()
            .dest(inst_dest)
            .offset(Point2::new(0.5, 0.5));

        graphics::draw(ctx, &inst_display, inst_params)?;
        Ok(())
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        match btn {
            Button::Start => self.resume = true,
            _ => (),
        }
    }
}

impl Scene for PauseScene {
    fn shared_state(&self) -> SharedState {
        self.state.clone()
    }

    fn get_transition(&mut self) -> Option<Transition> {
        if self.resume {
            return Some(Transition {
                transition_type: TransitionType::Pop,
                scene: None,
            });
        }
        None
    }
}

struct IntroScene {
    state: SharedState,
    start_game: bool,
}

impl IntroScene {
    fn new(state: &SharedState) -> Self {
        Self {
            state: state.clone(),
            start_game: false,
        }
    }
}

impl EventHandler for IntroScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let state = self.state.borrow();
        let (screen_w, screen_h) = graphics::size(ctx);
        let title_str = "Revenge of Cleveland Z";
        let title_display = graphics::Text::new((title_str, state.assets.font, 24.0));
        let title_dest = Point2::new(
            screen_w / 2.0 - title_display.width(ctx) as f32 / 2.0,
            screen_h / 2.0,
        );
        let title_params = graphics::DrawParam::default()
            .dest(title_dest)
            .offset(Point2::new(0.5, 0.5));
        graphics::draw(ctx, &title_display, title_params)?;

        let instructions = "Press Any Key";
        let inst_display = graphics::Text::new((instructions, state.assets.font, 18.0));
        let inst_dest = Point2::new(
            screen_w / 2.0 - inst_display.width(ctx) as f32 / 2.0,
            screen_h / 2.0 + 50.0,
        );
        let inst_params = graphics::DrawParam::default()
            .dest(inst_dest)
            .offset(Point2::new(0.5, 0.5));

        graphics::draw(ctx, &inst_display, inst_params)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.start_game = true;
    }
    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: Button, _id: GamepadId) {
        self.start_game = true;
    }
}

impl Scene for IntroScene {
    fn shared_state(&self) -> SharedState {
        self.state.clone()
    }

    fn get_transition(&mut self) -> Option<Transition> {
        if self.start_game {
            return Some(Transition {
                scene: Some(Box::new(GameScene::new(&self.shared_state()))),
                transition_type: TransitionType::Swap,
            });
        }
        None
    }
}

struct GameScene {
    state: SharedState,
    grid: GridState,
    input: InputState,
    falling: Option<Piece>,
    projection: Option<Piece>,
    held: Option<Piece>,
    next: VecDeque<Piece>,
    last_action: Instant,
    pause: bool,
    score: u32,
    level: u32,
    lines_cleared: u32,
}

impl GameScene {
    fn new(state: &SharedState) -> Self {
        Self {
            state: state.clone(),
            grid: GridState([[None; COLUMNS]; ROWS]),
            input: InputState::default(),
            falling: None,
            projection: None,
            held: None,
            next: VecDeque::from(vec![
                create_random_piece(BASE_SPEED),
                create_random_piece(BASE_SPEED),
                create_random_piece(BASE_SPEED),
            ]),
            last_action: Instant::now(),
            pause: false,
            score: 0,
            level: 1,
            lines_cleared: 0,
        }
    }

    fn swap_hold(&mut self) {
        if self.input.hold {
            self.input.hold = false;
            let held_piece = self.held.take();
            let falling_piece = self.falling.take();
            self.falling = held_piece;
            self.held = falling_piece;
        }
    }

    fn create_new_piece(&mut self) {
        match self.falling {
            Some(_) => (),
            None => {
                let next = self.next.pop_front().unwrap();
                let piece = create_random_piece(BASE_SPEED + self.level as f32 * SPEED_PER_LEVEL);
                self.next.push_back(piece);
                self.falling = Some(next);
            }
        }
    }

    fn should_end(&self) -> bool {
        for i in 0..2 {
            for j in 0..self.grid.0[i as usize].len() {
                if self.grid.0[i as usize][j].is_some() {
                    println!("no more space");
                    return true;
                }
            }
        }
        if let Some(p) = &self.falling {
            if !GameScene::is_valid_position(&self.grid, &p, &p.pos) {
                return true;
            }
        }
        false
    }

    fn clear_full_rows(&mut self) {
        let mut lines_cleared: u32 = 0;
        for i in 0..self.grid.0.len() {
            if self.grid.0[i].iter().all(|&x| x.is_some()) {
                for j in (0..(i + 1)).rev() {
                    for k in 0..self.grid.0[j].len() {
                        if j > 0 {
                            self.grid.0[j][k] = match self.grid.0[j - 1][k] {
                                Some(mut b) => {
                                    b.pos = Position::new(k as f32, j as f32);
                                    Some(b)
                                }
                                None => None,
                            }
                        } else {
                            self.grid.0[j][k] = None;
                        }
                    }
                }
                lines_cleared += 1;
            }
        }
        if lines_cleared > 0 {
            self.lines_cleared += lines_cleared;
            self.level = self.lines_cleared / 10 + 1;
            let raw_score = match lines_cleared {
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => panic!("Cleared more than 4 lines?"),
            };
            self.score += self.level * raw_score;
        }
    }

    fn is_valid_position(grid: &GridState, p: &Piece, pos: &Position) -> bool {
        for block in p.get_blocks(pos) {
            let pos = block.pos.grid_position();
            // check if we hit the bottom
            if pos.y as usize >= ROWS {
                return false;
            }
            if pos.x < 0 || pos.x as usize >= COLUMNS {
                return false;
            }
            match get_grid_idx(block.pos) {
                Some((x, y)) => {
                    // check if we're intersecting with a block
                    if let Some(_) = grid.0[y][x] {
                        return false;
                    }
                }
                None => (),
            };
        }
        true
    }

    fn update_piece_position(&mut self, dt: f32) {
        if let Some(ref mut p) = self.falling {
            let dv = p.velocity * dt;
            let pos = p.pos.absolute_position();

            let mut new_pos = Position::new(pos.x, pos.y + dv.y);

            if Instant::now() - self.last_action >= Duration::from_millis(MOVEMENT_DELAY) {
                if self.input.left {
                    let left = new_pos.move_left();
                    if GameScene::is_valid_position(&self.grid, p, &left) {
                        new_pos = left;
                    }
                }
                if self.input.right {
                    let right = new_pos.move_right();
                    if GameScene::is_valid_position(&self.grid, p, &right) {
                        new_pos = right;
                    }
                }
                if self.input.down {
                    let down = new_pos.move_down();
                    if GameScene::is_valid_position(&self.grid, p, &down) {
                        new_pos = down;
                    }
                }
                if self.input.moved() {
                    self.last_action = Instant::now();
                }
            }
            if Instant::now() - self.last_action >= Duration::from_millis(INPUT_DELAY) {
                if self.input.rotate_right {
                    p.rotate_cw();
                    if !GameScene::is_valid_position(&self.grid, p, &new_pos) {
                        p.rotate_ccw();
                    }
                }
                if self.input.rotate_left {
                    p.rotate_ccw();
                    if !GameScene::is_valid_position(&self.grid, p, &new_pos) {
                        p.rotate_cw();
                    }
                }
                if self.input.up {
                    let mut down = new_pos.move_down();
                    let mut count = 0;
                    while GameScene::is_valid_position(&self.grid, p, &down) {
                        count += 1;
                        new_pos = down;
                        down = new_pos.move_down();
                    }
                    p.landed = true;
                    self.score += count;
                }

                if self.input.acted() {
                    self.last_action = Instant::now();
                }
            }
            if p.pos.grid_position().y != new_pos.grid_position().y {
                if !GameScene::is_valid_position(&self.grid, p, &new_pos) {
                    // at this point, we landed on another block
                    new_pos = Position::new(
                        new_pos.absolute_position().x,
                        p.pos.grid_position().y as f32,
                    );
                    p.landed = true;
                }
            }
            p.pos = new_pos;

            if p.landed {
                for block in p.get_blocks(&p.pos) {
                    match get_grid_idx(block.pos) {
                        Some((x, y)) => {
                            self.grid.0[y][x] = Some(Block::from_piece(p, block.pos));
                        }
                        None => (),
                    }
                }
                self.falling = None;
                self.projection = None;
            }
        }
    }

    fn compute_projection_position(grid: &GridState, p: &Piece) -> Position {
        let mut down = p.pos.clone();
        let mut new_pos = down;
        while GameScene::is_valid_position(grid, p, &down) {
            new_pos = down;
            down = down.move_down();
        }
        new_pos
    }

    fn update_projection(&mut self) {
        if let Some(ref p) = self.falling {
            let mut proj = p.clone();
            proj.pos = GameScene::compute_projection_position(&self.grid, &proj);
            self.projection = Some(proj);
        }
    }
}

impl Scene for GameScene {
    fn shared_state(&self) -> SharedState {
        self.state.clone()
    }

    fn get_transition(&mut self) -> Option<Transition> {
        if self.should_end() {
            return Some(Transition {
                transition_type: TransitionType::Push,
                scene: Some(Box::new(GameOverScene::new(&self.shared_state()))),
            });
        } else if self.pause {
            self.pause = false;
            return Some(Transition {
                transition_type: TransitionType::Push,
                scene: Some(Box::new(PauseScene::new(&self.shared_state()))),
            });
        }
        None
    }
}

impl EventHandler for GameScene {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let shared_state = self.shared_state();
        self.swap_hold();
        self.create_new_piece();
        self.update_piece_position(shared_state.borrow().dt);
        self.clear_full_rows();
        self.update_projection();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let s = self.shared_state();
        let state = s.borrow();
        {
            let assets = &state.assets;
            {
                let _lock = graphics::use_shader(ctx, &state.shader);
                state.shader.send(ctx, state.opacity)?;

                if let Some(p) = &self.projection {
                    for b in p.get_blocks(&p.pos) {
                        draw_block(assets, ctx, b, state.screen_params)?;
                    }
                }
            }
            if let Some(p) = &self.falling {
                for b in p.get_blocks(&p.pos) {
                    draw_block(assets, ctx, b, state.screen_params)?;
                }
            }
            for i in 0..self.grid.0.len() {
                for j in 0..self.grid.0[i].len() {
                    if let Some(b) = self.grid.0[i][j] {
                        draw_block(assets, ctx, b, state.screen_params)?;
                    }
                }
            }
        }

        let box_position = Point2::new(state.screen_params.1, state.screen_params.2);
        graphics::draw(ctx, &state.border_box, (box_position,))?;

        let score_dest = Point2::new(10.0, 30.0);
        let score_str = format!("Score: {}", self.score);
        let score_display = graphics::Text::new((score_str, state.assets.font, 14.0));
        graphics::draw(ctx, &score_display, (score_dest, 0.0, graphics::WHITE))?;

        let level_dest = Point2::new(10.0, 50.0);
        let level_str = format!("Level: {}", self.level);
        let level_display = graphics::Text::new((level_str, state.assets.font, 14.0));
        graphics::draw(ctx, &level_display, (level_dest, 0.0, graphics::WHITE))?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::Space => self.pause = true,
            KeyCode::Left => self.input.left = true,
            KeyCode::Right => self.input.right = true,
            KeyCode::Z => self.input.rotate_left = true,
            KeyCode::X => self.input.rotate_right = true,
            KeyCode::Up => self.input.up = true,
            KeyCode::Down => self.input.down = true,
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left => self.input.left = false,
            KeyCode::Right => self.input.right = false,
            KeyCode::Z => self.input.rotate_left = false,
            KeyCode::X => self.input.rotate_right = false,
            KeyCode::Up => self.input.up = false,
            KeyCode::Down => self.input.down = false,
            _ => (),
        }
    }

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        match btn {
            Button::West => self.input.rotate_left = true,
            Button::South => self.input.rotate_right = true,
            Button::Start => self.pause = true,
            _ => (),
        }
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        match btn {
            Button::West => self.input.rotate_left = false,
            Button::South => self.input.rotate_right = false,
            _ => (),
        }
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _id: GamepadId) {
        match axis {
            Axis::DPadX => {
                if value < 0.0 {
                    self.input.left = true;
                } else if value > 0.0 {
                    self.input.right = true;
                } else {
                    self.input.left = false;
                    self.input.right = false;
                }
            }
            Axis::DPadY => {
                if value < 0.0 {
                    self.input.down = true;
                } else if value > 0.0 {
                    self.input.up = true;
                } else {
                    self.input.down = false;
                    self.input.up = false;
                }
            }
            _ => (),
        }
    }
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene + 'static>>,
    state: SharedState,
}

impl SceneManager {
    pub fn new(ctx: &mut Context) -> GameResult<SceneManager> {
        let state = ContextBoundState::new(ctx)?;
        let state_wrapper = Rc::new(RefCell::new(state));
        let sm = Self {
            scenes: vec![Box::new(IntroScene::new(&state_wrapper))],
            state: state_wrapper,
        };
        Ok(sm)
    }
}

struct ContextBoundState {
    assets: Assets,
    opacity: Opacity,
    screen_params: (f32, f32, f32, f32, f32),
    border_box: graphics::Mesh,
    shader: graphics::Shader<Opacity>,
    dt: f32,
    fps: f64,
}

impl ContextBoundState {
    pub fn new(ctx: &mut Context) -> GameResult<ContextBoundState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        let assets = Assets::new(ctx)?;

        let opacity = Opacity { pct: 0.05 };
        let shader = graphics::Shader::new(
            ctx,
            "/basic_150.glslv",
            "/opacity_150.glslf",
            opacity,
            "Opacity",
            None,
        )?;

        let screen_params = compute_screen_params(graphics::size(ctx));

        let s = Self {
            assets,
            shader,
            opacity,
            fps: 0.0,
            dt: 0.0,
            screen_params,
            border_box: build_border_box(ctx, screen_params.3, screen_params.4)?,
        };

        Ok(s)
    }
}

impl EventHandler for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            {
                let dt = 1.0 / (DESIRED_FPS as f32);
                let mut shared_state = self.state.borrow_mut();

                shared_state.fps = timer::fps(ctx);
                shared_state.dt = dt;
            }

            if let Some(s) = self.scenes.last_mut() {
                s.update(ctx)?;
                if let Some(transition) = s.get_transition() {
                    match transition.transition_type {
                        TransitionType::Pop => {
                            self.scenes.pop();
                            if self.scenes.len() == 0 {
                                ggez::event::quit(ctx);
                            }
                        }
                        TransitionType::Push => {
                            let new_s = transition.scene.unwrap();
                            self.scenes.push(new_s);
                        }
                        TransitionType::Swap => {
                            self.scenes.pop();
                            let new_s = transition.scene.unwrap();
                            self.scenes.push(new_s);
                        }
                        TransitionType::Reset => {
                            self.scenes.clear();
                            let new_s = transition.scene.unwrap();
                            self.scenes.push(new_s);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        for s in self.scenes.iter_mut() {
            s.draw(ctx)?;
        }

        let shared_state = self.state.borrow();

        let fps_dest = Point2::new(10.0, 10.0);
        let fps_str = format!("FPS: {:.2}", shared_state.fps);
        let fps_display = graphics::Text::new((fps_str, shared_state.assets.font, 14.0));
        graphics::draw(ctx, &fps_display, (fps_dest, 0.0, graphics::WHITE))?;

        graphics::present(ctx)?;

        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        if let Some(s) = self.scenes.last_mut() {
            s.key_down_event(ctx, keycode, keymod, repeat);
        };
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        if let Some(s) = self.scenes.last_mut() {
            s.key_up_event(ctx, keycode, keymod);
        };
    }

    fn gamepad_button_down_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        if let Some(s) = self.scenes.last_mut() {
            s.gamepad_button_down_event(ctx, btn, id);
        };
    }

    fn gamepad_button_up_event(&mut self, ctx: &mut Context, btn: Button, id: GamepadId) {
        if let Some(s) = self.scenes.last_mut() {
            s.gamepad_button_up_event(ctx, btn, id);
        };
    }

    fn gamepad_axis_event(&mut self, ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
        if let Some(s) = self.scenes.last_mut() {
            s.gamepad_axis_event(ctx, axis, value, id);
        };
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();

        let screen_params = compute_screen_params(graphics::size(ctx));
        let mut state = self.state.borrow_mut();
        state.screen_params = screen_params;
        state.border_box = build_border_box(ctx, screen_params.3, screen_params.4).unwrap();

        if let Some(s) = self.scenes.last_mut() {
            s.resize_event(ctx, width, height);
        };
    }
}

fn get_grid_idx(p: Position) -> Option<(usize, usize)> {
    let pos = p.grid_position();
    if pos.x < 0 || pos.x >= COLUMNS as i32 || pos.y < 0 || pos.y >= ROWS as i32 {
        return None;
    }
    Some((pos.x as usize, pos.y as usize))
}

fn draw_block(
    assets: &Assets,
    ctx: &mut Context,
    block: Block,
    screen_params: (f32, f32, f32, f32, f32),
) -> GameResult {
    let image = assets.get_image_for_block(&block);
    let (block_width, x_offset, y_offset, _, _) = screen_params;
    let scale = block_width / 160.0;

    let draw_params = graphics::DrawParam::new()
        .dest(block.pos.screen_coords(block_width, x_offset, y_offset))
        .scale(Vec2::new(scale, scale));
    graphics::draw(ctx, image, draw_params)
}

fn compute_screen_params(window_size: (f32, f32)) -> (f32, f32, f32, f32, f32) {
    let mut y_extent = (window_size.0 / ASPECT_RATIO).min(window_size.1);
    let y_offset = 0.5 * (window_size.1 - y_extent);
    let y_range = (y_offset, y_offset + y_extent);
    let block_height = ((y_range.1 - y_range.0) / ROWS as f32).trunc();
    y_extent = y_extent - (y_extent % block_height);

    let x_extent = block_height * COLUMNS as f32;
    let x_offset = 0.5 * (window_size.0 - x_extent);

    (
        block_height.trunc(),
        x_offset.trunc(),
        y_offset.trunc(),
        x_extent,
        y_extent,
    )
}

fn build_border_box(ctx: &mut Context, x_extent: f32, y_extent: f32) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Point2::new(0.0, 0.0),
            Point2::new(x_extent, 0.0),
            Point2::new(x_extent, y_extent),
            Point2::new(0.0, y_extent),
            Point2::new(0.0, 0.0),
        ],
        2.0,
        graphics::WHITE,
    )?;

    mb.build(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ggez::ContextBuilder;
    use std::env;
    use std::path;

    macro_rules! create_game_state {
        () => {{
            let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
                let mut path = path::PathBuf::from(manifest_dir);
                path.push("resources");
                path
            } else {
                println!("backup");
                path::PathBuf::from("./resources")
            };

            let cb = ContextBuilder::new("test", "test").add_resource_path(resource_dir);
            let (ctx, _) = &mut cb.build().unwrap();
            let sm = &mut SceneManager::new(ctx).unwrap();
            GameScene::new(&sm.state)
        }};
    }

    const DT: f32 = 1.0 / 60.0;

    #[test]
    fn create_new_piece_calls_if_none() {
        let mut state = create_game_state!();
        state.create_new_piece();
        assert!(state.falling.is_some());
    }

    #[test]
    fn no_falling_piece_at_start() {
        let state = create_game_state!();
        assert!(state.falling.is_none());
    }

    #[test]
    fn piece_position_falling() {
        let mut state = create_game_state!();
        state.create_new_piece();
        let initial_pos = state.falling.as_ref().unwrap().pos.clone();
        state.update_piece_position(DT);
        let p = state.falling.unwrap();

        assert_eq!(
            p.pos.absolute_position().y,
            initial_pos.absolute_position().y + p.velocity.y * DT
        );
        assert_eq!(
            p.pos.absolute_position().x,
            initial_pos.absolute_position().x
        );
    }

    #[test]
    fn piece_position_left_input() {
        let mut state = create_game_state!();
        state.create_new_piece();
        let initial_pos = state.falling.as_ref().unwrap().pos.clone();
        state.input.left = true;
        state.last_action = Instant::now() - Duration::from_millis(INPUT_DELAY);
        state.update_piece_position(DT);
        let p = state.falling.unwrap();

        assert_eq!(
            p.pos.absolute_position().y,
            initial_pos.absolute_position().y + p.velocity.y * DT
        );
        assert_eq!(
            p.pos.absolute_position().x,
            initial_pos.absolute_position().x - 1.0
        );
    }

    #[test]
    fn piece_position_right_input() {
        let mut state = create_game_state!();
        state.create_new_piece();
        let initial_pos = state.falling.as_ref().unwrap().pos.clone();
        state.input.right = true;
        state.last_action = Instant::now() - Duration::from_millis(INPUT_DELAY);
        state.update_piece_position(DT);
        let p = state.falling.unwrap();

        assert_eq!(
            p.pos.absolute_position().y,
            initial_pos.absolute_position().y + p.velocity.y * DT
        );
        assert_eq!(
            p.pos.absolute_position().x,
            initial_pos.absolute_position().x + 1.0
        );
    }

    #[test]
    fn piece_position_soft_down_input() {
        let mut state = create_game_state!();
        state.create_new_piece();
        let initial_pos = state.falling.as_ref().unwrap().pos.clone();
        state.input.down = true;
        state.last_action = Instant::now() - Duration::from_millis(INPUT_DELAY);
        state.update_piece_position(DT);
        let p = state.falling.unwrap();

        assert_eq!(
            p.pos.absolute_position().y,
            (initial_pos.absolute_position().y + p.velocity.y * DT).ceil()
        );
        assert_eq!(
            p.pos.absolute_position().x,
            initial_pos.absolute_position().x
        );
    }
}
