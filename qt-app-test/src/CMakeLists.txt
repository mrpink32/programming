qt5_add_resources(qt-app-test_SRCS resources.qrc)
add_executable(qt-app-test main.cpp ${qt-app-test_SRCS})
target_link_libraries(qt-app-test Qt5::Qml Qt5::Gui)

