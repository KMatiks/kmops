#!/bin/bash

########################################################################
# novnc doesn't like resolving DNS names, so we have to do it ourselves
########################################################################

# Resolve IP of kernel container
kernel=$(dig +short kernel)
# Resolve IP of novnc container
novnc=$(dig +short novnc)

./noVNC/utils/novnc_proxy --vnc "$kernel:5900" --listen "$novnc:6080"
