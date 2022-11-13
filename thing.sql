with validSubscriptions as (
    select * from subscriptions as s
    join users as u on u.subscription_id = s.id
    where u.status != false
),

mrt as (
    select distinct on (t.user_id) *
    from transactions as t
    where status = 'success'
    order by t.user_id, t.created_at desc
),

data as (
    select 
      s.user_id, 
      sum(
        case 
            when so.currency in ('usd', 'eur')
                case 
                    when s.interval = 'annual' then round(so.amount / 12)
                    when s.interval = 'quarterly' then round(so.amount / 3)
                    else so.amount
                end
            else
                case 
                    when s.interval = 'annual' then round((so.amount / 76) / 12)
                    when s.interval = 'quarterly' then round((so.amount / 76) / 3)
                    else so.amount
                end
        ) as sum_in_usd 
    from 
      validSubscriptions as s 
      join mrt on s.user_id = mrt.user_id 
      join stripe_orders as so on mrt.stripe_orders_id = so.id 
    where 
      s.subscription_status in ('active', 'past_due') 
)

select sum_in_usd from data;
