import uuid_rust

def generate_uuid():
    u1 = uuid_rust.MyUuid()
    print("v4", u1.to_string())
    u2 = uuid_rust.MyUuid.new_v7()
    print("v7", u2.to_string())
if __name__ == "__main__":
    generate_uuid()