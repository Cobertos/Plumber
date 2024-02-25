import bpy
from bpy.types import Collection

from ..plumber import UnknownEntity


def import_unknown_entity(entity: UnknownEntity, collection: Collection) -> None:
    name = f"{entity.class_name()}_{entity.id()}"

    obj = bpy.data.objects.new(name, object_data=None)
    obj.location = entity.position()
    obj.rotation_euler = entity.rotation()
    obj.scale = entity.scale()

    # Copy over all custom properties, obj is dict-like but does not implement
    # .update
    for k,v in entity.properties().items():
        obj[k] = v

    collection.objects.link(obj)
