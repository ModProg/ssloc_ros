cmake_minimum_required(VERSION 3.0)
project(ssloc_ros)


option(SSLOC-odas-msgs "Enable odas compatible messages" OFF)
option(SSLOC-audio_common_msgs-stamped "Enable stamped audio messages" OFF)
option(SSLOC-builtin-msgs "Enable builtin messages" ON)

if(SSLOC-odas-msgs)
  list(APPEND features -F odas-msgs)
endif()
if(SSLOC-audio_common_msgs-stamped)
  list(APPEND features -F audio_common_msgs-stamped)
endif()
if(SSLOC-builtin-msgs)
  list(APPEND features -F builtin-msgs)
endif()

if(NOT SSLOC-builtin-msgs)
  find_package(catkin REQUIRED COMPONENTS std_msgs ssloc_ros_msgs audio_common_msgs)
  catkin_package(CATKIN_DEPENDS message_runtime std_msgs ssloc_ros_msgs audio_common_msgs)
else()
  find_package(catkin)
  catkin_package(CATKIN_DEPENDS message_runtime)
endif()

add_custom_target(ssloc
   ALL
   COMMAND ${CMAKE_COMMAND} -E env ROSRUST_MSG_PATH=${CMAKE_CURRENT_SOURCE_DIR}/../../ env CARGO_HOME=${CMAKE_BINARY_DIR}/cargo cargo build --release --manifest-path ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml --target-dir ${CMAKE_BINARY_DIR}/cargo ${features}
   COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_BINARY_DIR}/cargo/release/ssloc_ros ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}/ssloc_ros
)

if (CATKIN_ENABLE_TESTING)
  find_package(roslaunch REQUIRED)
  roslaunch_add_file_check(launch)
endif()
