global_instance!(Input, input);

pub struct Input {
    //
}

pub fn init_input() {
    let input = Input {};

    set_instance(input);
}

impl Input {
    //
}

/*
class Input {
public:
Input();
~Input();

int32_t MouseX() const { return mouse_x_; }
int32_t MouseY() const { return mouse_y_; }
int32_t MouseWheel() const { return mouse_wheel_; }

bool IsButtonOn(int32_t key) const;
bool IsButtonPressed(int32_t key,
int32_t hold_frame = 0,
int32_t period_frame = 0) const;
bool IsButtonReleased(int32_t key) const;
void SetMouseVisible(int32_t is_visible);

bool IsMouseVisible() const { return is_mouse_visible_; }
void Update(Window* window, int32_t frame_count);

private:
SDL_GameController* gamepad1_;
SDL_GameController* gamepad2_;
SDL_Cursor* blank_cursor_;
SDL_Cursor* normal_cursor_;

int32_t frame_count_;
int32_t mouse_x_;
int32_t mouse_y_;
int32_t mouse_wheel_;
bool is_mouse_visible_;
int32_t key_state_[KEY_COUNT];

void UpdateKeyState(int32_t key, bool state);
};
*/
