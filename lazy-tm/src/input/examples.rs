/// Exemplos de uso dos padrões de projeto do módulo Input
/// Este arquivo demonstra diferentes formas de usar o sistema de input

use crate::controler::events::AppEvent;
use crate::input::*;
use ratatui::crossterm::event::KeyCode;
use std::io::Result;

/// Exemplo 1: Uso básico (compatível com código legado)
pub fn example_basic_usage() -> Result<()> {
    // Forma mais simples - mantém compatibilidade
    if let Some(event) = read_event()? {
        println!("Event received: {:?}", event);
    }
    Ok(())
}

/// Exemplo 2: Uso com InputConfig
pub fn example_with_input_config() -> Result<()> {
    let mut config = InputConfig::default();

    loop {
        if let Some(event) = config.read_event()? {
            match event {
                AppEvent::Quit => break,
                other => println!("Event: {:?}", other),
            }
        }
    }
    Ok(())
}

/// Exemplo 3: Keybindings customizados
pub fn example_custom_keybindings() -> Result<()> {
    // Cria bindings customizados usando Builder Pattern
    let custom_bindings = KeyBindings::new()
        .bind(KeyCode::Char('h'), AppEvent::SelPrevious)
        .bind(KeyCode::Char('l'), AppEvent::SelNext)
        .bind(KeyCode::Enter, AppEvent::ToggleTask)
        .bind(KeyCode::Char('n'), AppEvent::Add)
        .bind(KeyCode::Char('q'), AppEvent::Quit);

    // Cria reader com bindings customizados usando Factory Pattern
    let mut reader = InputReaderFactory::create_keyboard_with_bindings(custom_bindings);

    loop {
        if reader.has_event()? {
            if let Some(event) = reader.read()? {
                match event {
                    AppEvent::Quit => break,
                    other => println!("Custom binding event: {:?}", other),
                }
            }
        }
    }
    Ok(())
}

/// Exemplo 4: Usando diferentes presets
pub fn example_presets() -> Result<()> {
    // Preset Vim
    let _vim_config = InputConfig::from_type(InputReaderType::Vim);
    println!("Usando keybindings estilo Vim");

    // Preset Emacs
    let _emacs_config = InputConfig::from_type(InputReaderType::Emacs);
    println!("Usando keybindings estilo Emacs");

    // Preset Default
    let _default_config = InputConfig::from_type(InputReaderType::Default);
    println!("Usando keybindings padrão");

    Ok(())
}

/// Exemplo 5: KeyBindings Builder fluente
pub fn example_keybindings_builder() -> Result<()> {
    let bindings = KeyBindingsBuilder::new()
        .with_binding(KeyCode::Char('w'), AppEvent::SelPrevious)
        .with_binding(KeyCode::Char('s'), AppEvent::SelNext)
        .with_binding(KeyCode::Char('a'), AppEvent::SelPrevious)
        .with_binding(KeyCode::Char('d'), AppEvent::SelNext)
        .with_binding(KeyCode::Char(' '), AppEvent::ToggleTask)
        .build();

    println!("Bindings customizados construídos!");
    println!("Teclas mapeadas: {}", bindings.all_bindings().len());

    Ok(())
}

/// Exemplo 6: Modificando preset existente
pub fn example_modify_preset() -> Result<()> {
    // Começa com preset Vim e customiza
    let custom_bindings = KeyBindingsBuilder::from_preset(KeyBindings::vim_config())
        .with_binding(KeyCode::Char('x'), AppEvent::Delete)
        .without_binding(KeyCode::Char('d'))
        .build();

    let mut reader = InputReaderFactory::create_keyboard_with_bindings(custom_bindings);

    println!("Preset Vim customizado!");
    
    Ok(())
}

/// Exemplo 7: Usando InputConfigBuilder
pub fn example_input_config_builder() -> Result<()> {
    let reader = InputConfigBuilder::new()
        .reader_type(InputReaderType::Vim)
        .with_timeout(100)
        .build();

    println!("Reader criado com configuração customizada");
    println!("Reader type: {}", reader.name());

    Ok(())
}

/// Exemplo 8: Composite Reader (múltiplas fontes)
pub fn example_composite_reader() -> Result<()> {
    let mut composite = CompositeInputReader::new();

    // Adiciona múltiplos readers
    composite.add_reader(InputReaderFactory::create_keyboard());
    // No futuro, poderia adicionar mouse, rede, etc.
    
    println!("Composite reader com {} fontes", composite.len());

    // Lê do primeiro reader que tiver eventos
    if composite.has_event()? {
        if let Some(event) = composite.read()? {
            println!("Event from composite: {:?}", event);
        }
    }

    Ok(())
}

/// Exemplo 9: Extended Reader com estatísticas
pub fn example_extended_reader() -> Result<()> {
    let bindings = KeyBindings::default();
    let mut reader = ExtendedKeyboardReader::new(bindings);

    println!("Extended reader com tracking de estatísticas");

    // Após algumas interações (simulado)
    println!("Teclas pressionadas: {}", reader.key_count());
    if let Some(last) = reader.last_key() {
        println!("Última tecla: {:?}", last);
    }

    // Reseta estatísticas
    reader.reset_stats();
    println!("Estatísticas resetadas");

    Ok(())
}

/// Exemplo 10: Factory Pattern - criação a partir de string
pub fn example_factory_from_string() -> Result<()> {
    let configs = vec!["default", "vim", "emacs"];

    for config_name in configs {
        let reader = InputReaderFactory::create_from_config(config_name);
        println!("Criado reader: {} usando config '{}'", reader.name(), config_name);
    }

    Ok(())
}

/// Exemplo 11: Listando todos os tipos disponíveis
pub fn example_list_all_types() {
    println!("Tipos de Input Reader disponíveis:");
    for reader_type in InputReaderType::all() {
        println!("  - {}", reader_type.name());
    }
}

/// Exemplo 12: Criando reader com timeout customizado
pub fn example_custom_timeout() -> Result<()> {
    let reader = InputReaderFactory::create_with_timeout(100);
    println!("Reader criado com timeout de 100ms");
    println!("Timeout atual: {}ms", reader.timeout());

    Ok(())
}

/// Exemplo 13: Verificando bindings
pub fn example_check_bindings() {
    let bindings = KeyBindings::default();

    // Verifica se teclas específicas estão mapeadas
    println!("'q' está mapeado? {}", bindings.is_bound(KeyCode::Char('q')));
    println!("'x' está mapeado? {}", bindings.is_bound(KeyCode::Char('x')));

    // Lista todas as bindings
    println!("Total de bindings: {}", bindings.all_bindings().len());
}

/// Exemplo 14: Strategy Pattern - trocar reader em runtime
pub fn example_strategy_pattern() -> Result<()> {
    // Começa com reader default
    let mut current_reader: Box<dyn InputReader> = InputReaderFactory::create_keyboard();
    println!("Usando: {}", current_reader.name());

    // Troca para Vim durante execução
    current_reader = InputReaderFactory::create_vim_keyboard();
    println!("Trocado para: {}", current_reader.name());

    // Troca para Emacs
    current_reader = InputReaderFactory::create_emacs_keyboard();
    println!("Trocado para: {}", current_reader.name());

    Ok(())
}

/// Exemplo 15: Adapter Pattern - isolamento de dependências
pub fn example_adapter_pattern() -> Result<()> {
    // O KeyboardInputReader é um Adapter que isola o crossterm
    let reader = KeyboardInputReader::with_default_bindings();
    
    println!("KeyboardInputReader adapta crossterm para nossa interface");
    println!("Se precisarmos trocar de biblioteca, só mudamos o adapter");
    println!("Reader name: {}", reader.name());

    Ok(())
}

// Função helper para demonstrar todos os exemplos
pub fn run_all_examples() -> Result<()> {
    println!("=== Exemplos de Padrões de Projeto - Input Module ===\n");

    println!("Exemplo 11: Listando tipos disponíveis");
    example_list_all_types();
    println!();

    println!("Exemplo 13: Verificando bindings");
    example_check_bindings();
    println!();

    println!("Exemplo 4: Usando diferentes presets");
    example_presets()?;
    println!();

    println!("Exemplo 5: KeyBindings Builder");
    example_keybindings_builder()?;
    println!();

    println!("Exemplo 10: Factory de string");
    example_factory_from_string()?;
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_compile() {
        // Apenas testa que os exemplos compilam
        assert!(true);
    }
}
