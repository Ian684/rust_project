#[derive(Clone,Debug)]
enum Exercise22State{Idle,Running,Finished}

struct Exercise22Record{id:usize,label:String,score:i32,state:Exercise22State}

impl Exercise22Record{
    fn grade(&self)->char{
        match self.score{95..=100=>'A',85..=94=>'B',70..=84=>'C',_=>'D'}
    }
}

fn exercise_22_operation(values:&[i32])->i64{
    values.iter().enumerate().map(|(index,value)|(*value as i64)*(index as i64+22)).sum()
}

fn main(){
    println!("Exercise 22: strings");
    let records=[
        Exercise22Record{id:1,label:"Ada".into(),score:98,state:Exercise22State::Finished},
        Exercise22Record{id:2,label:"Linus".into(),score:86,state:Exercise22State::Running},
        Exercise22Record{id:3,label:"Grace".into(),score:73,state:Exercise22State::Idle},
    ];
    for record in &records{
        println!("{} {} {} {:?}",record.id,record.label,record.grade(),record.state);
    }
    println!("operation={}",exercise_22_operation(&[12,7,19,3,25]));
}

const EXERCISE_22_27:i32=22;
const EXERCISE_22_29:i32=22;
const EXERCISE_22_30:i32=22;
const EXERCISE_22_31:i32=22;
const EXERCISE_22_32:i32=22;
const EXERCISE_22_33:i32=22;
const EXERCISE_22_34:i32=22;
const EXERCISE_22_35:i32=22;
const EXERCISE_22_36:i32=22;
const EXERCISE_22_37:i32=22;
const EXERCISE_22_38:i32=22;
const EXERCISE_22_39:i32=22;
const EXERCISE_22_40:i32=22;
const EXERCISE_22_41:i32=22;
const EXERCISE_22_42:i32=22;
const EXERCISE_22_43:i32=22;
const EXERCISE_22_44:i32=22;
const EXERCISE_22_45:i32=22;
const EXERCISE_22_46:i32=22;
const EXERCISE_22_47:i32=22;
const EXERCISE_22_48:i32=22;
const EXERCISE_22_49:i32=22;
const EXERCISE_22_50:i32=22;
const EXERCISE_22_51:i32=22;
const EXERCISE_22_52:i32=22;
const EXERCISE_22_53:i32=22;
const EXERCISE_22_54:i32=22;
const EXERCISE_22_55:i32=22;
const EXERCISE_22_56:i32=22;
const EXERCISE_22_57:i32=22;
const EXERCISE_22_58:i32=22;
const EXERCISE_22_59:i32=22;
const EXERCISE_22_60:i32=22;
const EXERCISE_22_61:i32=22;
const EXERCISE_22_62:i32=22;
const EXERCISE_22_63:i32=22;
const EXERCISE_22_64:i32=22;
const EXERCISE_22_65:i32=22;
const EXERCISE_22_66:i32=22;
const EXERCISE_22_67:i32=22;
const EXERCISE_22_68:i32=22;
const EXERCISE_22_69:i32=22;
const EXERCISE_22_70:i32=22;
const EXERCISE_22_71:i32=22;
const EXERCISE_22_72:i32=22;
const EXERCISE_22_73:i32=22;
const EXERCISE_22_74:i32=22;
const EXERCISE_22_75:i32=22;
const EXERCISE_22_76:i32=22;
const EXERCISE_22_77:i32=22;
const EXERCISE_22_78:i32=22;
const EXERCISE_22_79:i32=22;
