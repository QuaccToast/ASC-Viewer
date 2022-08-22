Name:       rustdesk 
Version:    1.1.9
Release:    0
Summary:    RPM package
License:    GPL-3.0
Requires:   gtk3 libxcb libxdo libXfixes pulseaudio-libs alsa-lib cjkuni-uming-fonts python3-pip curl 

%description
The best open-source remote desktop client software, written in Rust. 

%prep
# we have no source, so nothing here

%build
# we have no source, so nothing here

%global __python %{__python3}

%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/lib/rustdesk/
mkdir -p %{buildroot}/usr/share/rustdesk/files/
install -m 755 $HBB/target/release/rustdesk %{buildroot}/usr/bin/rustdesk
install $HBB/libsciter-gtk.so %{buildroot}/usr/lib/rustdesk/libsciter-gtk.so
install $HBB/rustdesk.service %{buildroot}/usr/share/rustdesk/files/
install $HBB/256-no-margin.png %{buildroot}/usr/share/rustdesk/files/rustdesk.png
install $HBB/rustdesk.desktop %{buildroot}/usr/share/rustdesk/files/
install $HBB/pynput_service.py %{buildroot}/usr/share/rustdesk/files/

%files
/usr/bin/rustdesk
/usr/lib/rustdesk/libsciter-gtk.so
/usr/share/rustdesk/files/rustdesk.service
/usr/share/rustdesk/files/rustdesk.png
/usr/share/rustdesk/files/rustdesk.desktop
/usr/share/rustdesk/files/pynput_service.py
/usr/share/rustdesk/files/__pycache__/*

%changelog
# let's skip this for now

# https://www.cnblogs.com/xingmuxin/p/8990255.html
%pre
# can do something for centos7
case "$1" in
  1)
    # for install
  ;;
  2)
    # for upgrade
    systemctl stop rustdesk || true
  ;;
esac

%post
cp /usr/share/rustdesk/files/rustdesk.service /etc/systemd/system/rustdesk.service
cp /usr/share/rustdesk/files/rustdesk.desktop /usr/share/applications/
sudo -H pip3 install pynput
systemctl daemon-reload
systemctl enable rustdesk
systemctl start rustdesk
update-desktop-database

%preun
case "$1" in
  0)
    # for uninstall
    systemctl stop rustdesk || true
    systemctl disable rustdesk || true
    rm /etc/systemd/system/rustdesk.service || true
  ;;
  1)
    # for upgrade
  ;;
esac

%postun
case "$1" in
  0)
    # for uninstall
    rm /usr/share/applications/rustdesk.desktop || true
    update-desktop-database
  ;;
  1)
    # for upgrade
  ;;
esac
