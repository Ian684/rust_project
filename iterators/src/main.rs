#[derive(Clone,Debug)]
enum Exercise09State{Idle,Running,Finished}

struct Exercise09Record{id:usize,label:String,score:i32,state:Exercise09State}

impl Exercise09Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_09_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+9)).sum()
}

fn main(){
    println!("Exercise 09: iterators");
    let records=[
        Exercise09Record{id:1,label:"Ada".into(),score:98,state:Exercise09State::Finished},
        Exercise09Record{id:2,label:"Linus".into(),score:86,state:Exercise09State::Running},
        Exercise09Record{id:3,label:"Grace".into(),score:73,state:Exercise09State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_09_operation(&[12,7,19,3,25]));
}

const EXERCISE_09_27:i32=9;
const EXERCISE_09_29:i32=9;
const EXERCISE_09_30:i32=9;
const EXERCISE_09_31:i32=9;
const EXERCISE_09_32:i32=9;
const EXERCISE_09_33:i32=9;
const EXERCISE_09_34:i32=9;
const EXERCISE_09_35:i32=9;
const EXERCISE_09_36:i32=9;
const EXERCISE_09_37:i32=9;
const EXERCISE_09_38:i32=9;
const EXERCISE_09_39:i32=9;
const EXERCISE_09_40:i32=9;
const EXERCISE_09_41:i32=9;
const EXERCISE_09_42:i32=9;
const EXERCISE_09_43:i32=9;
const EXERCISE_09_44:i32=9;
const EXERCISE_09_45:i32=9;
const EXERCISE_09_46:i32=9;
const EXERCISE_09_47:i32=9;
const EXERCISE_09_48:i32=9;
const EXERCISE_09_49:i32=9;
const EXERCISE_09_50:i32=9;
const EXERCISE_09_51:i32=9;
const EXERCISE_09_52:i32=9;
const EXERCISE_09_53:i32=9;
const EXERCISE_09_54:i32=9;
const EXERCISE_09_55:i32=9;
const EXERCISE_09_56:i32=9;
const EXERCISE_09_57:i32=9;
const EXERCISE_09_58:i32=9;
const EXERCISE_09_59:i32=9;
const EXERCISE_09_60:i32=9;
const EXERCISE_09_61:i32=9;
const EXERCISE_09_62:i32=9;
const EXERCISE_09_63:i32=9;
const EXERCISE_09_64:i32=9;
const EXERCISE_09_65:i32=9;
const EXERCISE_09_66:i32=9;
const EXERCISE_09_67:i32=9;
const EXERCISE_09_68:i32=9;
const EXERCISE_09_69:i32=9;
const EXERCISE_09_70:i32=9;
const EXERCISE_09_71:i32=9;
const EXERCISE_09_72:i32=9;
const EXERCISE_09_73:i32=9;
const EXERCISE_09_74:i32=9;
const EXERCISE_09_75:i32=9;
const EXERCISE_09_76:i32=9;
const EXERCISE_09_77:i32=9;
const EXERCISE_09_78:i32=9;
const EXERCISE_09_79:i32=9;
