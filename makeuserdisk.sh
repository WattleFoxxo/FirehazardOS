# dd if=/dev/zero of=disk.img bs=1M count=256
# parted disk.img
# mktable gpt
# mkpart primary fat32 2048s 100%
# align-check optimal 1
# name 1 USER
# quit

rm sdc.img
dd if=/dev/zero of=sdc.img bs=1024 count=34000
mkfs.vfat -s 1 -F 32 -n "USER" -i 12345678 sdc.img

# losetup /dev/loop0 disk.img
mkdir /mnt/fhosuserdisk
mount -o loop sdc.img /mnt/fhosuserdisk
cp -r ./userdisk/* /mnt/fhosuserdisk/
umount /mnt/fhosuserdisk
rm -rf /mnt/fhosuserdisk
chmod 777 sdc.img
# losetup -d /dev/loop0