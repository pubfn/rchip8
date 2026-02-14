#include <QApplication>
#include <QPushButton>

extern "C" int chip8_qt_run() {
    int argc = 0;
    QApplication app(argc, nullptr);

    QPushButton button("CHIP-8 display (Qt via C++ bridge)");
    button.resize(420, 90);
    button.show();

    return app.exec();
}
