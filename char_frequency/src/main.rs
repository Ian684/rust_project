#[derive(Clone,Debug)]
enum Exercise32State{Idle,Running,Finished}

struct Exercise32Record{id:usize,label:String,score:i32,state:Exercise32State}

impl Exercise32Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_32_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+32)).sum()
}

fn main(){
    println!("Exercise 32: char_frequency");
    let records=[
        Exercise32Record{id:1,label:"Ada".into(),score:98,state:Exercise32State::Finished},
        Exercise32Record{id:2,label:"Linus".into(),score:86,state:Exercise32State::Running},
        Exercise32Record{id:3,label:"Grace".into(),score:73,state:Exercise32State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_32_operation(&[12,7,19,3,25]));
}

const EXERCISE_32_27:i32=32;
const EXERCISE_32_29:i32=32;
const EXERCISE_32_30:i32=32;
const EXERCISE_32_31:i32=32;
const EXERCISE_32_32:i32=32;
const EXERCISE_32_33:i32=32;
const EXERCISE_32_34:i32=32;
const EXERCISE_32_35:i32=32;
const EXERCISE_32_36:i32=32;
const EXERCISE_32_37:i32=32;
const EXERCISE_32_38:i32=32;
const EXERCISE_32_39:i32=32;
const EXERCISE_32_40:i32=32;
const EXERCISE_32_41:i32=32;
const EXERCISE_32_42:i32=32;
const EXERCISE_32_43:i32=32;
const EXERCISE_32_44:i32=32;
const EXERCISE_32_45:i32=32;
const EXERCISE_32_46:i32=32;
const EXERCISE_32_47:i32=32;
const EXERCISE_32_48:i32=32;
const EXERCISE_32_49:i32=32;
const EXERCISE_32_50:i32=32;
const EXERCISE_32_51:i32=32;
const EXERCISE_32_52:i32=32;
const EXERCISE_32_53:i32=32;
const EXERCISE_32_54:i32=32;
const EXERCISE_32_55:i32=32;
const EXERCISE_32_56:i32=32;
const EXERCISE_32_57:i32=32;
const EXERCISE_32_58:i32=32;
const EXERCISE_32_59:i32=32;
const EXERCISE_32_60:i32=32;
const EXERCISE_32_61:i32=32;
const EXERCISE_32_62:i32=32;
const EXERCISE_32_63:i32=32;
const EXERCISE_32_64:i32=32;
const EXERCISE_32_65:i32=32;
const EXERCISE_32_66:i32=32;
const EXERCISE_32_67:i32=32;
const EXERCISE_32_68:i32=32;
const EXERCISE_32_69:i32=32;
const EXERCISE_32_70:i32=32;
const EXERCISE_32_71:i32=32;
const EXERCISE_32_72:i32=32;
const EXERCISE_32_73:i32=32;
const EXERCISE_32_74:i32=32;
const EXERCISE_32_75:i32=32;
const EXERCISE_32_76:i32=32;
const EXERCISE_32_77:i32=32;
const EXERCISE_32_78:i32=32;
const EXERCISE_32_79:i32=32;
