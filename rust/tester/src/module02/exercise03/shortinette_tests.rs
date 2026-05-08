#[cfg(test)]
mod shortinette_tests_0203 {
    use ex03::{BuyError, Item, Player, SellError};
    use rand::{RngExt, seq::IndexedRandom};

    #[test]
    fn test_new_player_coins() {
        let mut rng = rand::rng();
        let coins: u8 = rng.random();

        let player = Player::new(coins);

        assert_eq!(
            player.coins(),
            coins,
            "Player::new({coins}) should initialize with exactly {coins} coins",
        );
    }

    #[test]
    fn test_new_player_item() {
        let mut rng = rand::rng();
        let coins: u8 = rng.random();
        let player = Player::new(coins);

        assert_eq!(
            player.item(),
            None,
            "A new player should start with an empty inventory (None), but an item was found"
        );
    }

    #[test]
    fn test_new_player_sell() {
        let mut rng = rand::rng();
        let coins: u8 = rng.random();
        let mut player = Player::new(coins);

        assert_eq!(
            player.sell(),
            Err(SellError::NoItemToSell),
            "Expected NoItemToSell error when a new player attempts to sell without an item"
        );
    }

    #[test]
    fn test_buy() {
        let items = [
            (Item::Sword, 10),
            (Item::Shield, 15),
            (Item::HealthPotion, 5),
            (Item::UpgradeStone, 25),
            (Item::Ring, 50),
        ];

        for (item, price) in items {
            let mut player = Player::new(price);
            let buy_result = player.buy(item);

            assert!(
                buy_result.is_ok(),
                "Player with {price} coins should be able to buy {item:?} (price: {price})",
            );

            assert_eq!(
                player.item(),
                Some(item),
                "After successful purchase, player inventory should contain {item:?}",
            );

            assert_eq!(
                player.coins(),
                0,
                "Player should have 0 coins left after spending exactly {price} coins on {item:?}",
            );
        }
    }

    #[test]
    fn test_sell() {
        let items = [
            (Item::Sword, 10),
            (Item::Shield, 15),
            (Item::HealthPotion, 5),
            (Item::UpgradeStone, 25),
            (Item::Ring, 50),
        ];

        for (item, price) in items {
            let mut player = Player::new(price);
            player
                .buy(item)
                .expect(&format!("Setup failed: Player could not buy {item:?}"));

            let sell_result = player.sell();

            assert!(
                sell_result.is_ok(),
                "Player should be able to sell the {item:?} they currently hold",
            );

            assert_eq!(
                player.item(),
                None,
                "Inventory should be None after selling {item:?}",
            );

            assert_eq!(
                player.coins(),
                price,
                "Player should have received {price} coins back from selling {item:?}",
            );
        }
    }

    #[test]
    fn test_buy_not_enough_coins() {
        let items = [
            (Item::Sword, 9),
            (Item::Shield, 14),
            (Item::HealthPotion, 4),
            (Item::UpgradeStone, 24),
            (Item::Ring, 49),
        ];

        for (item, price_minus_one) in items {
            let mut player = Player::new(price_minus_one);
            let buy_result = player.buy(item);

            assert_eq!(
                buy_result,
                Err(BuyError::NotEnoughCoins),
                "Purchase of {item:?} should fail when player only has {price_minus_one} coins",
            );

            assert_eq!(
                player.item(),
                None,
                "Inventory should remain empty after a failed purchase attempt of {item:?}",
            );

            assert_eq!(
                player.coins(),
                price_minus_one,
                "Player's {price_minus_one} coins should remain untouched after a failed purchase",
            );
        }
    }

    #[test]
    fn test_player_many_actions() {
        let items = [
            Item::Sword,
            Item::Shield,
            Item::HealthPotion,
            Item::UpgradeStone,
            Item::Ring,
        ];

        let mut rng = rand::rng();
        let starting_coins = u8::MAX;
        let mut player = Player::new(starting_coins);

        for i in 0..100 {
            let item_to_buy = items.choose(&mut rng).expect("Items list is empty");

            player.buy(*item_to_buy).unwrap_or_else(|_| {
                panic!(
                    "Action {i}: Player failed to buy {item_to_buy:?} with {} coins",
                    player.coins()
                )
            });

            player.sell().unwrap_or_else(|_| {
                panic!("Action {i}: Player failed to sell {item_to_buy:?} back to the shop",)
            });
        }

        assert_eq!(
            player.coins(),
            starting_coins,
            "Coin leak: Player started with {starting_coins} coins but ended with {}",
            player.coins()
        );
    }
}
