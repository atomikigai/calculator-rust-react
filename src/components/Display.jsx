import React, { Fragment } from 'react'
import './css/display.css'

const Display = (props) => {


    let result = props.value.toString().replaceAll(',','');
    result = result.replaceAll('=','');
    
    return(
        
        <div className="display">

            {

                result !== "" ? (
                    <Fragment>
                        <span className="operation " id="ope-color">
                            {
                                result
                            }
                        </span>
                        <span className="operation">
                            {props.final_result}
                        </span>
                    </Fragment>
                ):(
                    <span className="operation">
                        {0}
                    </span>
                )
            

            }



        </div>
    );

}

export default Display
