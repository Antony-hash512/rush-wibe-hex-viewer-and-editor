mod hex_view;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, CssProvider, gdk, 
          FileChooserNative, FileChooserAction};
use std::fs;

const APP_ID: &str = "org.gtk_rs.HexEditor";

fn main() {
    // Инициализируем GTK
    gtk::init().expect("Не удалось инициализировать GTK");
    
    // Создаем новое приложение
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    // Загружаем CSS стили
    load_css();
    
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    // Загружаем CSS из файла
    let provider = CssProvider::new();
    
    // Читаем содержимое CSS-файла
    match fs::read_to_string("src/style.css") {
        Ok(css) => {
            // Загружаем CSS содержимое
            provider.load_from_data(&css);
            
            // Применяем стили ко всему приложению
            gtk::style_context_add_provider_for_display(
                &gdk::Display::default().expect("Не удалось получить дисплей"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        },
        Err(e) => {
            eprintln!("Не удалось загрузить CSS файл: {}", e);
        }
    }
}

fn build_ui(app: &Application) {
    // Создаем главное окно
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hex Editor")
        .default_width(800)
        .default_height(600)
        .build();

    // Создаем вертикальный контейнер
    let vbox = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Создаем горизонтальный контейнер для заголовка и кнопки
    let header_box = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(12)
        .build();

    let title = Label::builder()
        .label("Hex Editor")
        .css_classes(vec!["title-1"])
        .build();
    header_box.append(&title);

    // Добавляем разделитель, чтобы кнопка была справа
    let separator = Box::new(gtk::Orientation::Horizontal, 0);
    separator.set_hexpand(true);
    header_box.append(&separator);

    let open_button = Button::builder()
        .label("Открыть файл")
        .build();

    header_box.append(&open_button);
    vbox.append(&header_box);

    let hex_view = hex_view::HexView::new();
    vbox.append(hex_view.get_container());

    // Получаем слабую ссылку на окно для использования в замыкании
    let window_weak = window.downgrade();
    
    let hex_view_clone = hex_view.clone();
    open_button.connect_clicked(move |_| {
        // Получаем сильную ссылку на окно из слабой
        let window = match window_weak.upgrade() {
            Some(window) => window,
            None => return,
        };
        
        // Используем FileChooserNative вместо FileChooserDialog
        // Это приведёт к использованию нативного диалога выбора файлов операционной системы
        let native_dialog = FileChooserNative::builder()
            .title("Выберите файл")
            .action(FileChooserAction::Open)
            .transient_for(&window)
            .modal(true)
            .build();
        
        // Рабочий каталог текущего процесса
        if let Err(e) = native_dialog.set_current_folder(Some(&gio::File::for_path("./"))) {
            eprintln!("Не удалось установить текущую папку: {}", e);
        }
        
        // Добавляем фильтр для всех файлов
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Все файлы"));
        filter.add_pattern("*");
        native_dialog.add_filter(&filter);
        
        let hex_view = hex_view_clone.clone();
        
        // Обрабатываем ответ от диалога
        native_dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        if let Some(path_str) = path.to_str() {
                            if let Err(e) = hex_view.load_file(path_str) {
                                let error_dialog = gtk::MessageDialog::builder()
                                    .text("Ошибка")
                                    .secondary_text(&format!("Не удалось открыть файл: {}", e))
                                    .message_type(gtk::MessageType::Error)
                                    .transient_for(&window)
                                    .build();
                                error_dialog.connect_response(|dialog, _| dialog.close());
                                error_dialog.show();
                            }
                        }
                    }
                }
            }
        });
        
        // Показываем диалог
        native_dialog.show();
    });

    // Добавляем контейнер в окно
    window.set_child(Some(&vbox));

    // Показываем окно
    window.present();
} 