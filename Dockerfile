FROM fedora:latest

ENV PROJECT_HOME=/src

ENV MINGW_GTKSOURCEVIEW4_FILENAME=mingw-w64-x86_64-gtksourceview4-4.8.3-1-any.pkg.tar.zst
ENV APP_NAME=mystudio-ide.exe

#
# Set up system
#
WORKDIR /root
RUN dnf -y update
RUN dnf clean all
RUN dnf install -y git cmake file gcc make man sudo tar zstd nano
RUN dnf install -y gcc-c++ boost boost-devel

#
# Add GtkSourceView
#
RUN dnf install -y gtksourceview4 gtksourceview4-devel

#
# Add Papirus icon
#
RUN dnf install -y papirus-icon-theme

#
# Build peldd to find dlls of exes
#
RUN git clone https://github.com/gsauthof/pe-util
WORKDIR pe-util
RUN git submodule update --init
RUN mkdir build

WORKDIR build
RUN cmake .. -DCMAKE_BUILD_TYPE=Release
RUN make

RUN mv /root/pe-util/build/peldd /usr/bin/peldd
RUN chmod +x /usr/bin/peldd

#
# Install Windows libraries
#
RUN dnf install -y mingw64-gcc 
RUN dnf install -y mingw64-freetype 
RUN dnf install -y mingw64-cairo 
RUN dnf install -y mingw64-harfbuzz 
RUN dnf install -y mingw64-pango 
RUN dnf install -y mingw64-poppler 
RUN dnf install -y mingw64-gtk3
RUN dnf install -y mingw64-winpthreads-static 
RUN dnf install -y mingw64-glib2-static 
RUN dnf install -y mingw64-libxml2-static
RUN dnf install -y mingw64-librsvg2-static

#
# Install rust
#
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup update

#
# Set up rust for cross compiling
#
RUN $HOME/.cargo/bin/rustup target add x86_64-pc-windows-gnu
ADD cargo-win.config $HOME/.cargo/config
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/sys-root/mingw/lib/pkgconfig/
ENV GTK_INSTALL_PATH=/usr/x86_64-w64-mingw32/sys-root/mingw/

#
# Setup the mount point
#
VOLUME $PROJECT_HOME
WORKDIR $PROJECT_HOME

#
# Add package.sh
#
ENV PACKAGED_DIR=release
ADD build-win.sh /usr/bin/package.sh
RUN chmod +x /usr/bin/package.sh


#
# Build and package executable
#
CMD ["/usr/bin/package.sh"]
