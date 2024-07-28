# Use a minimal Ubuntu base image
FROM ubuntu:latest

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive

# Add i386 architecture and install necessary packages
RUN dpkg --add-architecture i386 \
    && apt update \
    && apt install -y wget libc6:i386 libstdc++6:i386 libicu-dev

# Create directories and download SteamCMD
RUN mkdir /opt/steamcmd
WORKDIR /opt/steamcmd
RUN wget https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz
RUN tar -xvzf steamcmd_linux.tar.gz

# Create and run the update script
RUN echo "// update_scpsl.txt\n//\n@ShutdownOnFailedCommand 1 //set to 0 if updating multiple servers at once\n@NoPromptForPassword 1\nforce_install_dir /opt/scpsl/\n//for servers which don't need a login\nlogin anonymous\napp_update 996560 validate\nquit" > /opt/steamcmd/update_scpsl.txt
RUN /opt/steamcmd/steamcmd.sh +runscript /opt/steamcmd/update_scpsl.txt

EXPOSE 7777

# Change directory and run the SCP:SL server
WORKDIR /opt/scpsl
CMD echo "yes" | ./LocalAdmin