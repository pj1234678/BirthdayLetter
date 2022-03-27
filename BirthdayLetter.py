import os, zipfile, hashlib, hmac, struct, logging, random, json, time
from datetime import datetime, timedelta


TEMPLATES = {
    'U':"templateU.bin",
    'E':"templateE.bin",
    'J':"templateJ.bin",
    'K':"templateK.bin",
}

BUNDLEBASE = os.path.join(os.getcwd(), 'bundle')




def Hax():

    dt = datetime(2006, 11, 19) - timedelta(1)
    delta = (dt - datetime(2000, 1, 1))
    timestamp = delta.days * 86400 + delta.seconds
    print('Please Enter Wii Mac Address: (ex. 1111111111111111)')
    mac = input().lower()
    print('Please Enter Wii Region: (ex. U = US E = EUR J = JAP K = KOR)')
    template = TEMPLATES[input().upper()]
    print('Bundle with Hackmii? 1 = Yes 0 = No')
    bundle = bool(input())
    if mac == b"\x00\x17\xab\x99\x99\x99":
        print("If you're using Dolphin, try File->Open instead ;-).")
    Valid_Mac = False
    with open('oui_list.txt') as file:
        for line in file:
            if  (mac.startswith(line.rstrip().lower())):
                Valid_Mac = True
    if (Valid_Mac == True):
        print('Valid MAC Found')
    else:
        print('Invalid MAC Quitting...')
        quit()
    
    first_byte = bytes([int(mac[0:2], 16)])
    second_byte = bytes([int(mac[2:4], 16)])
    third_byte = bytes([int(mac[4:6], 16)])
    fourth_byte = bytes([int(mac[6:8], 16)])
    fifth_byte = bytes([int(mac[8:10], 16)])
    sixth_byte = bytes([int(mac[10:12], 16)])
    key = hashlib.sha1(first_byte + second_byte + third_byte + fourth_byte + fifth_byte + sixth_byte  + b"\x75\x79\x79").digest()
    blob = bytearray(open(os.path.join(os.getcwd(), template), 'rb').read())
    blob[0x08:0x10] = key[:8]
    blob[0xb0:0xc4] = bytes(20)
    blob[0x7c:0x80] = struct.pack(">I", timestamp)
    blob[0x80:0x8a] = (b"%010d" % timestamp)
    blob[0xb0:0xc4] = hmac.new(key[8:], bytes(blob), hashlib.sha1).digest()

    path = "private/wii/title/HAEA/%s/%s/%04d/%02d/%02d/%02d/%02d/HABA_#1/txt/%08X.000" % (
        key[:4].hex().upper(), key[4:8].hex().upper(),
        dt.year, dt.month-1, dt.day, dt.hour, dt.minute, timestamp
    )

    zip = zipfile.ZipFile('BirthdayLetter.zip', 'w')
    zip.writestr(path, blob)
    BUNDLE = [(name, os.path.join(BUNDLEBASE,name)) for name in os.listdir(BUNDLEBASE) if not name.startswith(".")]
    if bundle:
        for name, path in BUNDLE:
            zip.write(path, name)
    zip.close()

    print('Complete, Birthday Letter Generated as BirthdayLetter.zip.')
    print('Please Extract to the root of your SD Card')
    print('Dont Forget to set the time to November 19, 2006.')
    time.sleep(10) 
Hax()