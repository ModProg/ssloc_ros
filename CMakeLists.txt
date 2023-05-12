cmake_minimum_required(VERSION 3.0)
project(ssloc)

find_package(catkin REQUIRED COMPONENTS message_generation std_msgs)
add_message_files(
   DIRECTORY msg
   FILES
   Audio.msg
   UnitSsl.msg
   UnitSslArray.msg
   UnitSst.msg
   UnitSstArray.msg
)

generate_messages(DEPENDENCIES std_msgs)
catkin_package(CATKIN_DEPENDS message_runtime std_msgs)

add_custom_target(ssloc
   ALL
   COMMAND ${CMAKE_COMMAND} -E env ROSRUST_MSG_PATH=${CMAKE_CURRENT_SOURCE_DIR}/../../ cargo build --release --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${CMAKE_BINARY_DIR}/cargo
   COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_BINARY_DIR}/cargo/release/ssloc ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/ssloc
   COMMENT "Building ssloc!!!"
)

