# dd if=/dev/zero of=disk.img bs=1M count=256
# parted disk.img
# mktable gpt
# mkpart primary fat32 2048s 100%
# align-check optimal 1
# name 1 USER
# quit

rm sdc.img
dd if=/dev/zero of=sdc.img bs=512 count=131072 # 64MIB
mkfs.vfat -F 32 -s 2 sdc.img

# losetup /dev/loop0 disk.img
mkdir /mnt/fhosuserdisk
mount -o loop sdc.img /mnt/fhosuserdisk
cp -r ./userdisk/* /mnt/fhosuserdisk/
umount /mnt/fhosuserdisk
rm -rf /mnt/fhosuserdisk
chmod 777 sdc.img
# losetup -d /dev/loop0