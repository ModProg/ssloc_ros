# ssloc_ros

## Features
This package exposes a few features that can be enabled using cmake options using the pattern `catkin_make --cmake-args -DSSLOC-OPTION_NAME=ON` e.g. `-DSSLOC-odas-msgs=ON` and `-DSSLOC-builtin-msgs=OFF`.

1. `odas-msgs` enables the `odas/{ssl, sst, ssl_pcl2, sst_poses}` topics, but requires `odas_ros` to be available.
2. `audio_common_msgs-stamped` enables the `audio_stamped` topic, but requires a recent enough version of `audio_common_msgs` to be available.
3. `builtin-msgs` removes requirement for any external message packages to be available (even `ssloc_ros_msgs`) by using prebuilt message definitions. (also enables `odas-msgs` and `audio_common_msgs-stamped` but without their requirements). `ON` by default.
