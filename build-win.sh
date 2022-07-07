#!/bin/bash

#
# Extract mingw64-gtksourceview4 to MINGW64
#

cd /tmp


echo "Download and extract $MINGW_GTKSOURCEVIEW4_FILENAME"
curl https://repo.extreme-ix.org/msys2/mingw/mingw64/$MINGW_GTKSOURCEVIEW4_FILENAME -o $MINGW_GTKSOURCEVIEW4_FILENAME
tar --zstd -xf $MINGW_GTKSOURCEVIEW4_FILENAME mingw64 --transform 's/mingw64/mingw/'
cp -rf /tmp/mingw/** /usr/x86_64-w64-mingw32/sys-root/mingw/
rm -rf /tmp/$MINGW_GTKSOURCEVIEW4_FILENAME /tmp/mingw
echo "Finished.."

#
# Build app
#

echo "Building app.."
cd $PROJECT_HOME

$HOME/.cargo/bin/cargo build --target=x86_64-pc-windows-gnu --release

echo "Finished.."

#
# Package app
#

echo "Removing previous assets..."
rm -rf $PACKAGED_DIR
echo "Copying assets..."
mkdir -p $PACKAGED_DIR

# bail out if Windows binary doesn't exist
if [ ! -e target/x86_64-pc-windows-gnu/release/$APP_NAME ]
then
echo "File 'target/x86_64-pc-windows-gnu/release/$APP_NAME' not found, aborting"
exit 1
fi

# Copy exe
cp target/x86_64-pc-windows-gnu/release/$APP_NAME $PACKAGED_DIR/$APP_NAME

# Copy $APP_NAME required DLLs
/usr/bin/peldd $PACKAGED_DIR/$APP_NAME -t --ignore-errors | xargs cp -t $PACKAGED_DIR/
# Copy librsvg-2-2.dll to release/
cp -r $GTK_INSTALL_PATH/bin/librsvg-2-2.dll $PACKAGED_DIR/

# Asset dir: share
mkdir -p $PACKAGED_DIR/share/{glib-2.0/schemas,gtk-3.0,gtksourceview-4,themes}

# Asset dir: share/glib-2.0
cp -r $GTK_INSTALL_PATH/share/glib-2.0/schemas $PACKAGED_DIR/share/glib-2.0

# Asset dir: share/icons
cp -r $GTK_INSTALL_PATH/share/icons $PACKAGED_DIR/share/icons
cp -r /usr/share/icons/{hicolor,Papirus} $PACKAGED_DIR/share/icons

# Asset dir: lib
mkdir -p $PACKAGED_DIR/lib
cp -r $GTK_INSTALL_PATH/lib/gdk-pixbuf-2.0 $PACKAGED_DIR/lib

# Fix path inside loaders.cache
sed -i 's/\.\.\/lib/lib/g' $PACKAGED_DIR/lib/gdk-pixbuf-2.0/2.10.0/loaders.cache

# Copy gtksourceview assets
cp -r /usr/share/gtksourceview-4/** $PACKAGED_DIR/share/gtksourceview-4/

# Create GTK settings.ini file
cat << EOF > $PACKAGED_DIR/share/gtk-3.0/settings.ini
[Settings]
gtk-theme-name = Adwaita
gtk-icon-theme-name = Papirus
gtk-font-name = Segoe UI 10
gtk-xft-rgba = rgb
gtk-xft-antialias = 1
EOF

mingw-strip $PACKAGED_DIR/*.dll
mingw-strip $PACKAGED_DIR/$APP_NAME
echo "Finished.."

echo "Make zip file"
cd $PACKAGED_DIR
zip -9 -r -q "$APP_VERSION-win64.zip" .
echo "Finished"
