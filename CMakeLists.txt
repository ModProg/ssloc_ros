cmake_minimum_required(VERSION 3.0)
project(ssloc_ros)

find_package(catkin REQUIRED COMPONENTS std_msgs ssloc_ros_msgs audio_common_msgs)
catkin_package(CATKIN_DEPENDS message_runtime std_msgs ssloc_ros_msgs audio_common_msgs)

add_custom_target(ssloc
   ALL
   COMMAND ${CMAKE_COMMAND} -E env ROSRUST_MSG_PATH=${CMAKE_CURRENT_SOURCE_DIR}/../../ env CARGO_HOME=${CMAKE_BINARY_DIR}/cargo cargo build --release --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${CMAKE_BINARY_DIR}/cargo
   COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_BINARY_DIR}/cargo/release/ssloc_ros ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/ssloc_ros
)

if (CATKIN_ENABLE_TESTING)
  find_package(roslaunch REQUIRED)
  roslaunch_add_file_check(launch)
endif()