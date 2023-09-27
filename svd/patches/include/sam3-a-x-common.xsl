<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
    <!-- Trigger selection on ??? is missing the correct enumerated values. -->
    <xsl:template match="/device/peripherals/peripheral[name='DACC']/registers/register[name='MR']/fields/field[name='TRGSEL']">
        <field>
            <xsl:copy-of select="./name"/>
            <xsl:copy-of select="./description"/>
            <xsl:copy-of select="./bitOffset"/>
            <xsl:copy-of select="./bitWidth"/>
            <xsl:copy-of select="./access"/>
            <enumeratedValues>
                <name>TRGSEL_A</name>
                <enumeratedValue>
                    <name>EXTERNAL</name>
                    <description>External trigger</description>
                    <value>0x0</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>TIOOTCC0</name>
                    <description>TIO Output of the Timer Counter Channel 0</description>
                    <value>0x1</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>TIOOTCC1</name>
                    <description>TIO Output of the Timer Counter Channel 1</description>
                    <value>0x2</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>TIOOTCC2</name>
                    <description>TIO Output of the Timer Counter Channel 2</description>
                    <value>0x3</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>PWM0</name>
                    <description>PWM Event Line 0</description>
                    <value>0x4</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>PWM1</name>
                    <description>PWM Event Line 1</description>
                    <value>0x5</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>RESERVED</name>
                    <description>Reserved</description>
                    <value>0x6</value>
                </enumeratedValue>
                <enumeratedValue>
                    <name>RESERVED</name>
                    <description>Reserved</description>
                    <value>0x7</value>
                </enumeratedValue>
            </enumeratedValues>
        </field>
    </xsl:template>
    <xsl:template match="/device/peripherals/peripheral[name='DACC']/registers/register[name='CDR']/fields">
        <fields>
            <field>
                <name>CDR_HW0_DATA</name>
                <description>Data field of the lower CDR half-word</description>
                <bitOffset>0</bitOffset>
                <bitWidth>12</bitWidth>
                <access>write-only</access>
            </field>
            <field>
                <name>CDR_HW0_CHSEL</name>
                <description>Channel select field of the lower CDR half-word</description>
                <bitOffset>12</bitOffset>
                <bitWidth>2</bitWidth>
                <access>write-only</access>
                <enumeratedValues>
                    <enumeratedValue>
                        <name>CHANNEL0</name>
                        <description>Channel 0</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>CHANNEL1</name>
                        <description>Channel 1</description>
                        <value>1</value>
                    </enumeratedValue>
                </enumeratedValues>
            </field>
            <field>
                <name>CDR_HW1_DATA</name>
                <description>Data field of the upper CDR half-word</description>
                <bitOffset>16</bitOffset>
                <bitWidth>12</bitWidth>
                <access>write-only</access>
            </field>
            <field>
                <name>CDR_HW1_CHSEL</name>
                <description>Channel select field of the upper CDR half-word</description>
                <bitOffset>28</bitOffset>
                <bitWidth>2</bitWidth>
                <access>write-only</access>
                <enumeratedValues>
                    <enumeratedValue>
                        <name>CHANNEL0</name>
                        <description>Channel 0</description>
                        <value>0</value>
                    </enumeratedValue>
                    <enumeratedValue>
                        <name>CHANNEL1</name>
                        <description>Channel 1</description>
                        <value>1</value>
                    </enumeratedValue>
                </enumeratedValues>
            </field>
        </fields>
    </xsl:template>
</xsl:stylesheet>