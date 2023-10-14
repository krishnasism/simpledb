from simpledb import get_collection

collection = get_collection("documents")
collection.insert("krish", "hello")
print(collection.get("krish"))