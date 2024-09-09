cmake_minimum_required(VERSION 3.5)
project({{project-name}})

{% if template_type == "node" -%}

find_package(ament_cmake REQUIRED)

if(NOT DEFINED CMAKE_SUPPRESS_DEVELOPER_WARNINGS)
     set(CMAKE_SUPPRESS_DEVELOPER_WARNINGS 1 CACHE INTERNAL "No dev warnings")
endif()

include(r2r_cargo.cmake)

# put ros package dependencies here.
r2r_cargo(std_msgs               # just to test that it works
          rcl                    # we need the c ros2 api
          rcl_action             # as of r2r 0.1.0, we also need the action api
        {$ if use_additonal_msgs -%}
          {{additonal_msgs}}  # our custom message types
        {% endif -%}
        )

# install binaries
if(WIN32)
  set(SUFFIX ".exe")
else()
  set(SUFFIX "")
endif()

install(PROGRAMS
  ${CMAKE_SOURCE_DIR}/target/colcon/${PROJECT_NAME}${SUFFIX}
  DESTINATION lib/${PROJECT_NAME}
)

{% endif -%}

{% if template_type == "msgs" -%}

# Default to C99
if(NOT CMAKE_C_STANDARD)
  set(CMAKE_C_STANDARD 99)
endif()

# Default to C++14
if(NOT CMAKE_CXX_STANDARD)
  set(CMAKE_CXX_STANDARD 14)
endif()

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# find dependencies
find_package(ament_cmake REQUIRED)
find_package(rosidl_default_generators REQUIRED)

rosidl_generate_interfaces(${PROJECT_NAME}
  "msg/Hello.msg"
  "srv/HelloWorld.srv"
  "action/SayHello.action"
)

{% endif -%}

# we need this for ros/colcon
ament_package()
