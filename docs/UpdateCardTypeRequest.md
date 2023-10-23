# UpdateCardTypeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**icon_type** | Option<**i32**> | The type of the icon. 0 - system. 1 - user. | [optional]
**icon_id** | Option<**i32**> | An icon for the card type. If set to 0, the card type will not have an icon. | [optional]
**name** | Option<**String**> | The name for the card type. | [optional]
**description** | Option<**String**> | A description of the new card type. | [optional]
**color** | Option<**String**> | The color of the card type. 6 hexadecimal characters are expected. | [optional]
**card_color_sync** | Option<**i32**> | When set to 1 the color of the cards with this type will be replaced with the color of the type and cannot be changed manually. | [optional]
**all_properties_are_locked** | Option<**i32**> | Controls whether the icon_type, icon_id, color and card_color_sync properties can be modified per board. | [optional]
**availability** | Option<**i32**> | When set to 0 the card type has to be added to boards manually. Every workspace manager can add it to the boards they can manage. When set to 1 the card type is added automatically to all new boards but workspace managers can remove it from the boards they can manage. When set to 2 the card type is added automatically to all boards and cannot be removed. | [optional]
**is_enabled** | Option<**i32**> | Controls whether this card type is enabled. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


