# Domain Models

## Menu

```rust
impl Menu {
    fn create() -> Menu { ... }
    fn add_dinner(dinner: Dinner) { ... }
    fn remove_dinner(dinner: Dinner) { ... }
    fn update_section(section: MenuSection) { ... }
}
```

```json
{
    "id": "00000000-0000-0000-0000-000000000000",
    "hostId": "00000000-0000-0000-0000-000000000000",
    "name": "Dummy Menu",
    "description": "A Menu with dummy food",
    "averageRating": 4.5,
    "sections": [
        {
            "id": "00000000-0000-0000-0000-000000000000",
            "name": "Appetizers",
            "description": "Starters",
            "items": [
                {
                    "id": "00000000-0000-0000-0000-000000000000",
                    "name": "Fried Pickles",
                    "description": "Deep fried pickles",
                    "price": 5.99
                }
            ]
        }
    ],
    "createdDateTime": "2020-01-01T00:00:00.0000000Z",
    "updatedDateTime": "2020-01-01T00:00:00.0000000Z",
    "dinnerIds": [
        "00000000-0000-0000-0000-000000000000"
    ],
    "menuReviewIds": [
        "00000000-0000-0000-0000-000000000000"
    ],
}
```