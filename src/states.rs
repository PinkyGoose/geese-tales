use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// Главные состояния приложения, объединяющие разные группы
pub enum AppState {
    MainMenu,
    Game,      // Здесь будут игровые под-состояния
    LoadGame,  // Окно загрузки сохранений
    PauseMenu, // Экран паузы с под-состояниями
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// Под-состояния, относящиеся к основному игровому процессу
pub enum GameState {
    Exploration,
    Combat,
    Dialogue,
    Inventory,
    CharacterCustomization,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// Под-состояния, относящиеся к настройкам новой игры
pub enum NewGameState {
    SeedSelection,
    DifficultySelection,
    CharacterCreation,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// Под-состояния экрана паузы
pub enum PauseMenuState {
    MainPause, // Основное меню паузы (сохранить игру, выйти, настройки)
    Settings,  // Настройки в паузе
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// Под-состояния главного экрана
pub enum MainMenuState {
    MainScreen, // Основное меню паузы (сохранить игру, выйти, настройки)
    Settings,   // Настройки в паузе
    NewGame,    // Настройка новой игры (сид, сложность и т.п.)
}
