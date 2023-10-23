# CreateStickerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**icon_type** | Option<**i32**> | The type of the icon. 0 - system. 1 - user. | [optional][default to 0]
**icon_id** | Option<**i32**> | An icon for the sticker. If set to 0, the sticker will not have an icon. | [optional][default to 0]
**label** | **String** | A label for the new sticker. | 
**color** | Option<**String**> | The color of the sticker. 6 hexadecimal characters are expected. | [optional]
**availability** | Option<**i32**> | When set to 0 the sticker has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the sticker is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the sticker is added automatically to all boards and cannot be removed. | [optional][default to Variant0]
**is_enabled** | Option<**i32**> | Controls whether this sticker is enabled. | [optional][default to Variant1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


