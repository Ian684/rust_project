#[derive(Clone,Debug)]
enum Exercise31State{Idle,Running,Finished}

struct Exercise31Record{id:usize,label:String,score:i32,state:Exercise31State}

impl Exercise31Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_31_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+31)).sum()
}

fn main(){
    println!("Exercise 31: bank");
    let records=[
        Exercise31Record{id:1,label:"Ada".into(),score:98,state:Exercise31State::Finished},
        Exercise31Record{id:2,label:"Linus".into(),score:86,state:Exercise31State::Running},
        Exercise31Record{id:3,label:"Grace".into(),score:73,state:Exercise31State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_31_operation(&[12,7,19,3,25]));
}

const EXERCISE_31_27:i32=31;
const EXERCISE_31_29:i32=31;
const EXERCISE_31_30:i32=31;
const EXERCISE_31_31:i32=31;
const EXERCISE_31_32:i32=31;
const EXERCISE_31_33:i32=31;
const EXERCISE_31_34:i32=31;
const EXERCISE_31_35:i32=31;
const EXERCISE_31_36:i32=31;
const EXERCISE_31_37:i32=31;
const EXERCISE_31_38:i32=31;
const EXERCISE_31_39:i32=31;
const EXERCISE_31_40:i32=31;
const EXERCISE_31_41:i32=31;
const EXERCISE_31_42:i32=31;
const EXERCISE_31_43:i32=31;
const EXERCISE_31_44:i32=31;
const EXERCISE_31_45:i32=31;
const EXERCISE_31_46:i32=31;
const EXERCISE_31_47:i32=31;
const EXERCISE_31_48:i32=31;
const EXERCISE_31_49:i32=31;
const EXERCISE_31_50:i32=31;
const EXERCISE_31_51:i32=31;
const EXERCISE_31_52:i32=31;
const EXERCISE_31_53:i32=31;
const EXERCISE_31_54:i32=31;
const EXERCISE_31_55:i32=31;
const EXERCISE_31_56:i32=31;
const EXERCISE_31_57:i32=31;
const EXERCISE_31_58:i32=31;
const EXERCISE_31_59:i32=31;
const EXERCISE_31_60:i32=31;
const EXERCISE_31_61:i32=31;
const EXERCISE_31_62:i32=31;
const EXERCISE_31_63:i32=31;
const EXERCISE_31_64:i32=31;
const EXERCISE_31_65:i32=31;
const EXERCISE_31_66:i32=31;
const EXERCISE_31_67:i32=31;
const EXERCISE_31_68:i32=31;
const EXERCISE_31_69:i32=31;
const EXERCISE_31_70:i32=31;
const EXERCISE_31_71:i32=31;
const EXERCISE_31_72:i32=31;
const EXERCISE_31_73:i32=31;
const EXERCISE_31_74:i32=31;
const EXERCISE_31_75:i32=31;
const EXERCISE_31_76:i32=31;
const EXERCISE_31_77:i32=31;
const EXERCISE_31_78:i32=31;
const EXERCISE_31_79:i32=31;
