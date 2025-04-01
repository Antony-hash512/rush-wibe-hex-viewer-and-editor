mod hex_view;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, FileChooserDialog, FileChooserAction};
use gio::prelude::*;

const APP_ID: &str = "org.gtk_rs.HexEditor";

fn main() {
    // Создаем новое приложение
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();
    
    app.connect_activate(build_ui);
    app.run();
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

    let hex_view_clone = hex_view.clone();
    open_button.connect_clicked(move |_| {
        let dialog = FileChooserDialog::builder()
            .title("Выберите файл")
            .action(FileChooserAction::Open)
            .build();

        dialog.add_button("Отмена", gtk::ResponseType::Cancel);
        dialog.add_button("Открыть", gtk::ResponseType::Accept);

        let hex_view = hex_view_clone.clone();
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        if let Some(path_str) = path.to_str() {
                            if let Err(e) = hex_view.load_file(path_str) {
                                let error_dialog = gtk::MessageDialog::builder()
                                    .text("Ошибка")
                                    .secondary_text(&format!("Не удалось открыть файл: {}", e))
                                    .message_type(gtk::MessageType::Error)
                                    .build();
                                error_dialog.connect_response(|dialog, _| dialog.close());
                                error_dialog.show();
                            }
                        }
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    });

    // Добавляем контейнер в окно
    window.set_child(Some(&vbox));

    // Показываем окно
    window.present();
} 