use super::system_prelude::*;
use deathframe::physics::query::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default)]
pub struct UpdateReactiveAnimationsSystem;

impl<'a> System<'a> for UpdateReactiveAnimationsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationsContainer<ReactiveAnimationKey>>,
        ReadStorage<'a, TriggerReactiveAnimation>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animations_store,
            trigger_reactive_store,
            collider_store,
        ): Self::SystemData,
    ) {
        let mut triggerable_animations_ids = Vec::new();

        let mut triggerable_animations: HashMap<
            Index,
            &mut AnimationsContainer<ReactiveAnimationKey>,
        > = (&entities, &mut animations_store)
            .join()
            .map(|(entity, animation)| {
                let id = entity.id();
                triggerable_animations_ids.push(id);
                (id, animation)
            })
            .collect();

        let mut trigger_animations_for: Vec<(Index, AnimationAction)> =
            Vec::new();

        let collider_queries: HashMap<
            AnimationAction,
            QueryExpression<CollisionTag>,
        > = {
            use deathframe::physics::query::exp::prelude_variants::*;

            AnimationAction::iter()
                .map(|action| (action, IsState(action.into())))
                .collect()
        };

        for (_, trigger_collider) in
            (&trigger_reactive_store, &collider_store).join()
        {
            for animation_action in AnimationAction::iter() {
                let colliding_entities = trigger_collider
                    .query::<FilterQuery<CollisionTag>>()
                    .filter_ids(&triggerable_animations_ids)
                    .exp(collider_queries.get(&animation_action).unwrap())
                    .run();
                for colliding_entity in colliding_entities {
                    trigger_animations_for
                        .push((colliding_entity.id, animation_action));
                }
            }
        }

        for (entity_id, animation_action) in trigger_animations_for {
            if let Some(animations) = triggerable_animations.get_mut(&entity_id)
            {
                let animation_key: ReactiveAnimationKey =
                    animation_action.into();
                animations.truncate_animation_stack(2);
                let _ = animations.push(animation_key);
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AnimationAction {
    Enter,
    Steady,
    Leave,
}

impl Into<QueryValueState> for AnimationAction {
    fn into(self) -> QueryValueState {
        match self {
            Self::Enter => QueryValueState::Enter,
            Self::Steady => QueryValueState::Steady,
            Self::Leave => QueryValueState::Leave,
        }
    }
}

impl Into<ReactiveAnimationKey> for AnimationAction {
    fn into(self) -> ReactiveAnimationKey {
        match self {
            Self::Enter => ReactiveAnimationKey::OnEnter,
            Self::Steady => ReactiveAnimationKey::OnSteady,
            Self::Leave => ReactiveAnimationKey::OnLeave,
        }
    }
}

impl AnimationAction {
    fn iter() -> AnimationActionIter {
        AnimationActionIter::default()
    }
}

#[derive(Default)]
struct AnimationActionIter(u8);

impl Iterator for AnimationActionIter {
    type Item = AnimationAction;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        match self.0 {
            1 => Some(AnimationAction::Enter),
            2 => Some(AnimationAction::Steady),
            3 => Some(AnimationAction::Leave),
            _ => None,
        }
    }
}
