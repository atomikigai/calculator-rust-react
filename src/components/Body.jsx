import React, {useEffect, useState} from 'react'
import './css/body.css'
import Display from './Display'
import Nums from './Nums'
import { invoke } from '@tauri-apps/api/tauri'


const Body = () => {

    const [valueNum, setvalueNum] = useState([]);

    const [final_result, setFinalResult] = useState("");

    const cleanDisplay = () =>{
        setvalueNum([]);
        setFinalResult("");
    }

    
    const resultclick = () =>{
        let valores = "";
        valores = valueNum.toString().replaceAll(',','');
        valores = valores.replaceAll('=','');
        invoke('calculator', { invokeMessage: valores}).then((message) => setFinalResult(message));
    }


    let num = ['AC', '+/-', '%', '÷',
     '7', '8', '9', 'x', 
     '4', '5', '6', '-',
     '1', '2', '3', '+',
     '⏎', '0', '.', '='
    ];

    let contador = 0;


    return(
        
        <div className="body">

            <div className="d-body">
                <Display value={valueNum} final_result={final_result}/>
            </div>
        
            <div className="buttons">
                
                {
                    num.map((item, index) => {
                        contador += 1
                        return <Nums key={index} value={item} count={contador} setvalueNum={setvalueNum}
                            valueNum={valueNum} cleanDisplay={cleanDisplay} result={resultclick}
                        />
                    }) 
                }

            </div>

        

        </div>
    );

}

export default Body
