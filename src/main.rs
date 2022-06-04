slint::slint! {
import { LineEdit, CheckBox, HorizontalBox, VerticalBox } from "std-widgets.slint";

export global Logic := {
    callback change_text(string) -> string;
}

export MainWindow := Window {
    width: 400px;
    height: 400px;
    background: #1e1e1e;
    HorizontalBox {
        VerticalBox {
            input := LineEdit {
                placeholder-text: "Next thing I need to do...";
                height: 30px;
            }
            Flickable {
                interactive: false;
                VerticalBox {
                    new_item := Rectangle {
                        area := TouchArea {}
                        background: area.pressed ? #333333 : #1e1e1e;
                        height: 30px;
                        VerticalBox {
                            Text {
                                text: { Logic.change_text(input.text); }
                                color: #ffffff;
                            }
                        }
                    }
                }
            }
        }
    }
}}


fn main() {
    let main_window = MainWindow::new();

    main_window.global::<Logic>().on_change_text(|string| { string.as_str().into() });

    main_window.run();
}
