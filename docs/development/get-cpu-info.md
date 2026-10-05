

adb shell getprop ro.product.model
adb shell getprop ro.build.version.release
adb shell getprop ro.build.version.sdk
adb shell getprop ro.soc.model
adb shell cat /proc/cpuinfo
adb shell cat /proc/meminfo | head

---
rustc --version --verbose
cargo --version
uname -a

---
getprop ro.product.model
getprop ro.build.version.release
getprop ro.build.version.sdk
getprop ro.soc.model
cat /proc/cpuinfo
cat /proc/meminfo | head -n 5

---
getprop ro.product.manufacturer
getprop ro.product.device
getprop ro.hardware
getprop ro.board.platform
getprop ro.build.fingerprint
nproc

---
getprop ro.build.version.release
cat /sys/devices/system/cpu/present
cat /sys/devices/system/cpu/possible
cat /sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq 2>/dev/null
grep -E 'MemTotal|MemAvailable' /proc/meminfo