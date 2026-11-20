#[derive(Clone,Debug)]
enum Exercise20State{Idle,Running,Finished}

struct Exercise20Record{id:usize,label:String,score:i32,state:Exercise20State}

impl Exercise20Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_20_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+20)).sum()
}

fn main(){
    println!("Exercise 20: slices");
    let records=[
        Exercise20Record{id:1,label:"Ada".into(),score:98,state:Exercise20State::Finished},
        Exercise20Record{id:2,label:"Linus".into(),score:86,state:Exercise20State::Running},
        Exercise20Record{id:3,label:"Grace".into(),score:73,state:Exercise20State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_20_operation(&[12,7,19,3,25]));
}

const EXERCISE_20_27:i32=20;
const EXERCISE_20_29:i32=20;
const EXERCISE_20_30:i32=20;
const EXERCISE_20_31:i32=20;
const EXERCISE_20_32:i32=20;
const EXERCISE_20_33:i32=20;
const EXERCISE_20_34:i32=20;
const EXERCISE_20_35:i32=20;
const EXERCISE_20_36:i32=20;
const EXERCISE_20_37:i32=20;
const EXERCISE_20_38:i32=20;
const EXERCISE_20_39:i32=20;
const EXERCISE_20_40:i32=20;
const EXERCISE_20_41:i32=20;
const EXERCISE_20_42:i32=20;
const EXERCISE_20_43:i32=20;
const EXERCISE_20_44:i32=20;
const EXERCISE_20_45:i32=20;
const EXERCISE_20_46:i32=20;
const EXERCISE_20_47:i32=20;
const EXERCISE_20_48:i32=20;
const EXERCISE_20_49:i32=20;
const EXERCISE_20_50:i32=20;
const EXERCISE_20_51:i32=20;
const EXERCISE_20_52:i32=20;
const EXERCISE_20_53:i32=20;
const EXERCISE_20_54:i32=20;
const EXERCISE_20_55:i32=20;
const EXERCISE_20_56:i32=20;
const EXERCISE_20_57:i32=20;
const EXERCISE_20_58:i32=20;
const EXERCISE_20_59:i32=20;
const EXERCISE_20_60:i32=20;
const EXERCISE_20_61:i32=20;
const EXERCISE_20_62:i32=20;
const EXERCISE_20_63:i32=20;
const EXERCISE_20_64:i32=20;
const EXERCISE_20_65:i32=20;
const EXERCISE_20_66:i32=20;
const EXERCISE_20_67:i32=20;
const EXERCISE_20_68:i32=20;
const EXERCISE_20_69:i32=20;
const EXERCISE_20_70:i32=20;
const EXERCISE_20_71:i32=20;
const EXERCISE_20_72:i32=20;
const EXERCISE_20_73:i32=20;
const EXERCISE_20_74:i32=20;
const EXERCISE_20_75:i32=20;
const EXERCISE_20_76:i32=20;
const EXERCISE_20_77:i32=20;
const EXERCISE_20_78:i32=20;
const EXERCISE_20_79:i32=20;
