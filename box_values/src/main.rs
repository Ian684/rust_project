#[derive(Clone,Debug)]
enum Exercise04State{Idle,Running,Finished}

struct Exercise04Record{id:usize,label:String,score:i32,state:Exercise04State}

impl Exercise04Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_04_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+4)).sum()
}

fn main(){
    println!("Exercise 04: box_values");
    let records=[
        Exercise04Record{id:1,label:"Ada".into(),score:98,state:Exercise04State::Finished},
        Exercise04Record{id:2,label:"Linus".into(),score:86,state:Exercise04State::Running},
        Exercise04Record{id:3,label:"Grace".into(),score:73,state:Exercise04State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_04_operation(&[12,7,19,3,25]));
}

const EXERCISE_04_27:i32=4;
const EXERCISE_04_29:i32=4;
const EXERCISE_04_30:i32=4;
const EXERCISE_04_31:i32=4;
const EXERCISE_04_32:i32=4;
const EXERCISE_04_33:i32=4;
const EXERCISE_04_34:i32=4;
const EXERCISE_04_35:i32=4;
const EXERCISE_04_36:i32=4;
const EXERCISE_04_37:i32=4;
const EXERCISE_04_38:i32=4;
const EXERCISE_04_39:i32=4;
const EXERCISE_04_40:i32=4;
const EXERCISE_04_41:i32=4;
const EXERCISE_04_42:i32=4;
const EXERCISE_04_43:i32=4;
const EXERCISE_04_44:i32=4;
const EXERCISE_04_45:i32=4;
const EXERCISE_04_46:i32=4;
const EXERCISE_04_47:i32=4;
const EXERCISE_04_48:i32=4;
const EXERCISE_04_49:i32=4;
const EXERCISE_04_50:i32=4;
const EXERCISE_04_51:i32=4;
const EXERCISE_04_52:i32=4;
const EXERCISE_04_53:i32=4;
const EXERCISE_04_54:i32=4;
const EXERCISE_04_55:i32=4;
const EXERCISE_04_56:i32=4;
const EXERCISE_04_57:i32=4;
const EXERCISE_04_58:i32=4;
const EXERCISE_04_59:i32=4;
const EXERCISE_04_60:i32=4;
const EXERCISE_04_61:i32=4;
const EXERCISE_04_62:i32=4;
const EXERCISE_04_63:i32=4;
const EXERCISE_04_64:i32=4;
const EXERCISE_04_65:i32=4;
const EXERCISE_04_66:i32=4;
const EXERCISE_04_67:i32=4;
const EXERCISE_04_68:i32=4;
const EXERCISE_04_69:i32=4;
const EXERCISE_04_70:i32=4;
const EXERCISE_04_71:i32=4;
const EXERCISE_04_72:i32=4;
const EXERCISE_04_73:i32=4;
const EXERCISE_04_74:i32=4;
const EXERCISE_04_75:i32=4;
const EXERCISE_04_76:i32=4;
const EXERCISE_04_77:i32=4;
const EXERCISE_04_78:i32=4;
const EXERCISE_04_79:i32=4;
